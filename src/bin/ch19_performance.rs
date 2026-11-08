//! # Chapitre 19 — Performance
//!
//! Règle n°1 : **mesurer avant d'optimiser**. Règle n°2 : toujours en `--release`.
//!
//! Lancer : `cargo run --release --bin ch19_performance`
//!
//! Les chiffres varient selon la machine ; ce qui compte, ce sont les ORDRES DE GRANDEUR
//! et les raisons derrière. Pour des mesures sérieuses : `criterion` ou `divan`, `cargo flamegraph`,
//! `perf`, `valgrind --tool=cachegrind`.

#![allow(clippy::needless_range_loop)]

use std::hint::black_box;
use std::time::{Duration, Instant};

/// Mesure la médiane de plusieurs exécutions (plus robuste qu'une seule mesure).
fn mesurer<R>(nom: &str, repetitions: u32, mut f: impl FnMut() -> R) -> Duration {
    let mut durees: Vec<Duration> = (0..repetitions)
        .map(|_| {
            let debut = Instant::now();
            // `black_box` empêche le compilateur d'éliminer le calcul « inutile ».
            black_box(f());
            debut.elapsed()
        })
        .collect();
    durees.sort();
    let mediane = durees[durees.len() / 2];
    println!("  {nom:<42} {mediane:>12.2?}");
    mediane
}

fn main() {
    if cfg!(debug_assertions) {
        println!("⚠ Build DEBUG : les mesures n'ont pas de sens. Relancez avec --release.\n");
    }

    allocations();
    iterateurs();
    cache_memoire();
    dispatch();
    disposition_memoire();
}

// ───────────────────────── 1. Allocations ─────────────────────────

fn allocations() {
    println!("--- Allocations ---");
    const N: usize = 1_000_000;

    let sans = mesurer("Vec::new() puis push × 1M", 5, || {
        let mut v = Vec::new();
        for i in 0..N {
            v.push(i);
        }
        v
    });
    let avec = mesurer("Vec::with_capacity(1M) puis push × 1M", 5, || {
        let mut v = Vec::with_capacity(N);
        for i in 0..N {
            v.push(i);
        }
        v
    });
    println!(
        "  → with_capacity ≈ {:.1}× plus rapide",
        sans.as_secs_f64() / avec.as_secs_f64().max(1e-12)
    );

    mesurer("format! dans une boucle (10k allocations)", 5, || {
        let mut s = String::new();
        for i in 0..10_000 {
            s = s + &format!("{i},");
        }
        s
    });
    mesurer("write! dans un String réutilisé", 5, || {
        use std::fmt::Write;
        let mut s = String::with_capacity(60_000);
        for i in 0..10_000 {
            let _ = write!(s, "{i},");
        }
        s
    });
}

// ───────────────────────── 2. Itérateurs vs boucles ─────────────────────────

fn iterateurs() {
    println!("--- Itérateurs : abstraction à coût zéro ---");
    let donnees: Vec<u64> = (0..5_000_000).collect();

    mesurer("boucle for avec index", 7, || {
        let mut somme = 0u64;
        for i in 0..donnees.len() {
            if donnees[i] % 3 == 0 {
                somme += donnees[i] * 2;
            }
        }
        somme
    });
    mesurer("chaîne d'itérateurs filter/map/sum", 7, || {
        donnees
            .iter()
            .filter(|&&x| x % 3 == 0)
            .map(|&x| x * 2)
            .sum::<u64>()
    });
    // En release les deux compilent vers un code quasi identique (et vectorisé) ;
    // les itérateurs évitent en plus les vérifications de bornes `donnees[i]`.
}

// ───────────────────────── 3. Localité du cache ─────────────────────────

fn cache_memoire() {
    println!("--- Cache : l'ordre d'accès compte ---");
    const T: usize = 3_000;
    let matrice: Vec<u32> = (0..(T * T) as u32).collect(); // row-major : ligne après ligne

    let lignes = mesurer("parcours par lignes (séquentiel)", 5, || {
        let mut s = 0u64;
        for i in 0..T {
            for j in 0..T {
                s += u64::from(matrice[i * T + j]);
            }
        }
        s
    });
    let colonnes = mesurer("parcours par colonnes (sauts de 12 Ko)", 5, || {
        let mut s = 0u64;
        for j in 0..T {
            for i in 0..T {
                s += u64::from(matrice[i * T + j]);
            }
        }
        s
    });
    println!(
        "  → même nombre d'opérations, colonnes ≈ {:.1}× plus lent (défauts de cache)",
        colonnes.as_secs_f64() / lignes.as_secs_f64().max(1e-12)
    );
}

// ───────────────────────── 4. Dispatch statique vs dynamique ─────────────────────────

trait Operation {
    fn appliquer(&self, x: u64) -> u64;
}
struct Ajoute(u64);
struct Double;
impl Operation for Ajoute {
    #[inline]
    fn appliquer(&self, x: u64) -> u64 {
        x.wrapping_add(self.0)
    }
}
impl Operation for Double {
    #[inline]
    fn appliquer(&self, x: u64) -> u64 {
        x.wrapping_mul(2)
    }
}

fn boucle_statique<O: Operation>(op: &O, n: u64) -> u64 {
    (0..n).fold(1, |acc, _| op.appliquer(black_box(acc)))
}
fn boucle_dynamique(op: &dyn Operation, n: u64) -> u64 {
    (0..n).fold(1, |acc, _| op.appliquer(black_box(acc)))
}

fn dispatch() {
    println!("--- Dispatch statique (génériques) vs dynamique (dyn) ---");
    let n = 50_000_000;
    mesurer("générique : appel inliné", 5, || {
        boucle_statique(&Ajoute(3), n)
    });
    mesurer("dyn Trait : appel via vtable", 5, || {
        boucle_dynamique(&Ajoute(3), n)
    });
    // Résultat souvent identique ici : comme le type concret est visible à l'appel, LLVM
    // « dévirtualise » l'appel `dyn` et l'inline. Le surcoût réel apparaît quand le type
    // concret n'est PAS connu localement (collection hétérogène, plugins) : appel indirect via
    // la vtable, pas d'inlining, moins de vectorisation. Essayez avec un `Vec<Box<dyn Operation>>`.
    let _ = Double;
}

// ───────────────────────── 5. Disposition en mémoire ─────────────────────────

#[allow(dead_code)]
struct Naif {
    a: u8,
    b: u64,
    c: u8,
    d: u32,
}

#[allow(dead_code)]
#[repr(C)] // ordre des champs imposé, comme en C → du padding apparaît
struct EnC {
    a: u8,
    b: u64,
    c: u8,
    d: u32,
}

fn disposition_memoire() {
    println!("--- Disposition mémoire (size_of / align_of) ---");
    println!(
        "  Naif  (repr(Rust), champs réordonnés) : {} octets",
        size_of::<Naif>()
    );
    println!(
        "  EnC   (repr(C), ordre a,b,c,d)        : {} octets",
        size_of::<EnC>()
    );
    println!(
        "  Option<Box<i32>>  = {} (même taille que Box<i32> = {} : « niche »)",
        size_of::<Option<Box<i32>>>(),
        size_of::<Box<i32>>()
    );
    println!(
        "  Option<u32>       = {} octets (pas de niche : un tag est ajouté)",
        size_of::<Option<u32>>()
    );
    println!(
        "  &str = {} octets (pointeur + longueur) ; &[u8] = {} ; Box<dyn Trait> = {}",
        size_of::<&str>(),
        size_of::<&[u8]>(),
        size_of::<Box<dyn Operation>>()
    );
    println!(
        "  Vec<u8> = {} octets (pointeur + capacité + longueur) ; String = {}",
        size_of::<Vec<u8>>(),
        size_of::<String>()
    );
    println!("  align_of::<u64>() = {}", align_of::<u64>());
}
