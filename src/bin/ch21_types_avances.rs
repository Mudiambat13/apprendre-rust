//! # Chapitre 21 — Types et système de types avancés
//!
//! Génériques constants, `const fn`, HRTB, `Send`/`Sync`, niches mémoire, `Pin`,
//! `Drop` et ordre de destruction, `impl Trait` dans les alias.
//!
//! Lancer : `cargo run --bin ch21_types_avances`

#![allow(dead_code, clippy::needless_range_loop)]

use std::any::type_name;
use std::cell::RefCell;
use std::marker::PhantomPinned;
use std::num::NonZeroU32;
use std::pin::Pin;
use std::rc::Rc;

fn main() {
    const_generics();
    const_fn();
    hrtb();
    send_sync();
    niches();
    pin();
    ordre_de_destruction();
}

// ───────────── Génériques constants : des VALEURS comme paramètres de type ─────────────

#[derive(Debug, Clone, Copy, PartialEq)]
struct Matrice<const L: usize, const C: usize>([[i32; C]; L]);

impl<const L: usize, const C: usize> Matrice<L, C> {
    fn transposee(&self) -> Matrice<C, L> {
        let mut r = [[0; L]; C];
        for i in 0..L {
            for j in 0..C {
                r[j][i] = self.0[i][j];
            }
        }
        Matrice(r)
    }

    /// Produit matriciel : le compilateur vérifie que les dimensions sont compatibles.
    fn produit<const K: usize>(&self, autre: &Matrice<C, K>) -> Matrice<L, K> {
        let mut r = [[0; K]; L];
        for i in 0..L {
            for j in 0..K {
                for k in 0..C {
                    r[i][j] += self.0[i][k] * autre.0[k][j];
                }
            }
        }
        Matrice(r)
    }
}

fn somme_tableau<const N: usize>(t: [i32; N]) -> i32 {
    t.iter().sum()
}

fn const_generics() {
    println!("--- Génériques constants ---");
    let a = Matrice([[1, 2, 3], [4, 5, 6]]); // 2×3
    let b = a.transposee(); //                  3×2
    println!("{:?}", a.produit(&b)); //         2×2
    // a.produit(&a); // ← ne compile pas : 2×3 × 2×3 est impossible
    println!("{} {}", somme_tableau([1, 2, 3]), somme_tableau([10; 5]));
}

// ───────────── const fn : calcul à la compilation ─────────────

const fn factorielle(n: u64) -> u64 {
    let mut r = 1;
    let mut i = 2;
    while i <= n {
        r *= i;
        i += 1;
    }
    r
}

const fn table_des_carres<const N: usize>() -> [u32; N] {
    let mut t = [0; N];
    let mut i = 0;
    while i < N {
        t[i] = (i * i) as u32;
        i += 1;
    }
    t
}

const FACT_10: u64 = factorielle(10); // calculé par le compilateur
static CARRES: [u32; 8] = table_des_carres::<8>();

// Une assertion à la compilation : le programme ne compile pas si elle est fausse.
const _: () = assert!(factorielle(5) == 120);
const _: () = assert!(size_of::<u64>() == 8);

fn const_fn() {
    println!("--- const fn ---");
    println!("10! = {FACT_10} ; carrés = {CARRES:?}");
}

// ───────────── HRTB : Higher-Ranked Trait Bounds ─────────────

/// `for<'a>` : « pour TOUT lifetime 'a ». La closure doit fonctionner avec n'importe quelle
/// référence, y compris celles créées DANS la fonction.
fn appliquer_a_locale<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let locale = String::from("  texte local  ");
    f(&locale).len() // `locale` vit moins longtemps que n'importe quel 'a venant de l'extérieur
}

fn hrtb() {
    println!("--- HRTB ---");
    println!("{}", appliquer_a_locale(|s| s.trim()));
}

// ───────────── Send et Sync : traits « auto » ─────────────

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn send_sync() {
    println!("--- Send / Sync ---");
    // Send : on peut DÉPLACER la valeur vers un autre thread.
    // Sync : on peut PARTAGER &T entre threads (T: Sync ⟺ &T: Send).
    assert_send::<String>();
    assert_sync::<String>();
    assert_send::<std::sync::Arc<i32>>();
    // assert_send::<Rc<i32>>();          // ← ne compile pas : le compteur de Rc n'est pas atomique
    // assert_sync::<RefCell<i32>>();     // ← ne compile pas : mutation sans synchronisation
    assert_send::<RefCell<i32>>(); // en revanche on peut le déplacer
    println!(
        "  ok : String est Send+Sync ; Rc n'est ni l'un ni l'autre ; RefCell est Send mais pas Sync"
    );
    println!("  type_name::<Rc<i32>>() = {}", type_name::<Rc<i32>>());
}

// ───────────── Niches : optimisations de disposition ─────────────

fn niches() {
    println!("--- Niches ---");
    // Une référence/Box/NonZero n'est jamais « 0 » : `None` utilise cette valeur interdite.
    println!(
        "  &i32 = {} ; Option<&i32> = {}",
        size_of::<&i32>(),
        size_of::<Option<&i32>>()
    );
    println!(
        "  NonZeroU32 = {} ; Option<NonZeroU32> = {}",
        size_of::<NonZeroU32>(),
        size_of::<Option<NonZeroU32>>()
    );
    println!(
        "  u32 = {} ; Option<u32> = {}",
        size_of::<u32>(),
        size_of::<Option<u32>>()
    );
    println!(
        "  bool = {} ; Option<bool> = {}",
        size_of::<bool>(),
        size_of::<Option<bool>>()
    );
}

// ───────────── Pin : empêcher un déplacement en mémoire ─────────────

/// Une struct auto-référentielle : `pointeur` pointe vers `donnees`, dans la MÊME struct.
/// Si on la déplaçait, `pointeur` deviendrait un pointeur pendant. `Pin` interdit ce déplacement.
/// C'est exactement le cas des futures générées par `async` (chapitre 14).
struct AutoReferent {
    donnees: String,
    pointeur: *const String,
    _epingle: PhantomPinned, // rend le type `!Unpin`
}

impl AutoReferent {
    fn new(texte: &str) -> Pin<Box<Self>> {
        let mut boite = Box::pin(AutoReferent {
            donnees: texte.to_string(),
            pointeur: std::ptr::null(),
            _epingle: PhantomPinned,
        });
        let ptr: *const String = &boite.donnees;
        // SAFETY: on ne déplace pas la valeur ; on modifie seulement un champ.
        unsafe { boite.as_mut().get_unchecked_mut().pointeur = ptr };
        boite
    }

    fn via_pointeur(self: Pin<&Self>) -> &str {
        // SAFETY: la valeur est épinglée, donc `pointeur` reste valide.
        unsafe { &*self.pointeur }
    }
}

fn pin() {
    println!("--- Pin ---");
    let a = AutoReferent::new("auto-référent");
    println!("  {} (via le pointeur interne)", a.as_ref().via_pointeur());
    // let b = *a;  // ← ne compile pas : on ne peut pas sortir une valeur `!Unpin` d'un Pin
}

// ───────────── Drop : ordre de destruction ─────────────

struct Trace {
    nom: &'static str,
    journal: Rc<RefCell<Vec<&'static str>>>,
}

impl Drop for Trace {
    fn drop(&mut self) {
        self.journal.borrow_mut().push(self.nom);
    }
}

struct Paire {
    _a: Trace,
    _b: Trace,
}

fn ordre_de_destruction() {
    println!("--- Ordre de destruction ---");
    let journal = Rc::new(RefCell::new(Vec::new()));
    let nouveau = |nom| Trace {
        nom,
        journal: Rc::clone(&journal),
    };
    {
        let _x = nouveau("x");
        let _y = nouveau("y");
        let _paire = Paire {
            _a: nouveau("champ a"),
            _b: nouveau("champ b"),
        };
        let _ = nouveau("jeté tout de suite"); // `_` ne lie rien : détruit immédiatement
    }
    // Variables locales : ordre INVERSE de déclaration. Champs d'une struct : ordre de déclaration.
    println!("  {:?}", journal.borrow());
}
