//! # Chapitre 9 — Closures et itérateurs
//!
//! Lancer : `cargo run --bin ch09_closures_iterateurs`

#![allow(clippy::type_complexity)]

use std::collections::HashMap;

fn main() {
    closures();
    iterateurs_de_base();
    adaptateurs();
    collecte();
    iterateur_perso();
}

// ───────────────────────── Closures ─────────────────────────

fn closures() {
    println!("--- Closures ---");

    // Syntaxe : |paramètres| expression. Types inférés.
    let ajouter = |a, b| a + b;
    println!("{}", ajouter(2, 3));

    // Capture de l'environnement — le compilateur choisit le mode le moins restrictif :
    let mot = String::from("rust");
    let longueur = || mot.len(); // capture par référence  → Fn
    println!("{} {}", longueur(), mot); // `mot` toujours utilisable

    let mut compteur = 0;
    let mut incrementer = || {
        compteur += 1; // capture par référence mutable → FnMut
        compteur
    };
    incrementer();
    incrementer();
    println!("compteur = {compteur}");

    let proprietaire = String::from("consommé");
    let consommer = move || proprietaire; // capture par valeur & la renvoie → FnOnce
    println!("{}", consommer());
    // consommer(); // ← ERREUR : FnOnce ne peut être appelée qu'une fois

    // Les 3 traits : FnOnce ⊃ FnMut ⊃ Fn (tout `Fn` est aussi `FnMut` et `FnOnce`).
    fn appliquer<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(x)
    }
    fn appliquer_deux_fois(mut f: impl FnMut()) {
        f();
        f();
    }
    let facteur = 3;
    println!("{}", appliquer(|x| x * facteur, 7));
    let mut n = 0;
    appliquer_deux_fois(|| n += 10);
    println!("n = {n}");

    // Retourner une closure : `impl Fn` (statique) ou `Box<dyn Fn>` (dynamique).
    fn multiplicateur(k: i32) -> impl Fn(i32) -> i32 {
        move |x| x * k
    }
    let operations: Vec<(&str, Box<dyn Fn(i32) -> i32>)> = vec![
        ("double", Box::new(|x| x * 2)),
        ("carré", Box::new(|x| x * x)),
        ("triple", Box::new(multiplicateur(3))),
    ];
    for (nom, op) in &operations {
        println!("  {nom}(5) = {}", op(5));
    }
}

// ───────────────────────── Itérateurs ─────────────────────────

fn iterateurs_de_base() {
    println!("--- Itérateurs ---");
    let v = vec![1, 2, 3];

    // Trois façons d'itérer :
    let _a: Vec<&i32> = v.iter().collect(); //            &T
    let _b: Vec<i32> = v.clone().into_iter().collect(); //  T  (consomme)
    let mut v2 = v.clone();
    v2.iter_mut().for_each(|x| *x += 1); //               &mut T
    println!("{v2:?}");

    // Les itérateurs sont PARESSEUX : rien ne s'exécute avant un consommateur.
    let _inutile = v.iter().map(|x| {
        println!("jamais affiché");
        x * 2
    });

    // Consommateurs : sum, product, count, min, max, fold, any, all, find, position, last, nth…
    println!(
        "sum={} prod={} max={:?} min={:?}",
        v.iter().sum::<i32>(),
        v.iter().product::<i32>(),
        v.iter().max(),
        v.iter().min()
    );
    println!(
        "{}",
        v.iter().fold(String::new(), |acc, x| format!("{acc}{x}"))
    );
    println!(
        "any>2: {}  all>0: {}  find>1: {:?}  position(3): {:?}",
        v.iter().any(|&x| x > 2),
        v.iter().all(|&x| x > 0),
        v.iter().find(|&&x| x > 1),
        v.iter().position(|&x| x == 3)
    );
}

fn adaptateurs() {
    println!("--- Adaptateurs ---");
    let nombres = 1..=10;

    let resultat: Vec<i32> = nombres
        .clone()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect();
    println!("carrés des pairs : {resultat:?}");

    println!(
        "zip      : {:?}",
        [1, 2, 3].iter().zip(["a", "b", "c"]).collect::<Vec<_>>()
    );
    println!(
        "enumerate: {:?}",
        ["x", "y"].iter().enumerate().collect::<Vec<_>>()
    );
    println!(
        "skip/take: {:?}",
        nombres.clone().skip(2).take(3).collect::<Vec<_>>()
    );
    println!("step_by  : {:?}", (0..20).step_by(5).collect::<Vec<_>>());
    println!("chain    : {:?}", (1..3).chain(7..9).collect::<Vec<_>>());
    println!("rev      : {:?}", (1..4).rev().collect::<Vec<_>>());
    println!(
        "flat_map : {:?}",
        ["ab", "cd"]
            .iter()
            .flat_map(|s| s.chars())
            .collect::<String>()
    );
    println!(
        "take_while: {:?}",
        nombres.clone().take_while(|&n| n < 4).collect::<Vec<_>>()
    );
    println!(
        "scan (cumul): {:?}",
        (1..=5)
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<_>>()
    );
    println!(
        "windows  : {:?}",
        [1, 2, 4, 8]
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>()
    );
    println!(
        "filter_map: {:?}",
        ["1", "x", "3"]
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>()
    );

    let mut it = [1, 2, 3].iter().peekable();
    while let Some(x) = it.next() {
        match it.peek() {
            Some(suivant) => print!("{x}→{suivant} "),
            None => println!("{x} (fin)"),
        }
    }

    let mots = ["pomme", "kiwi", "banane"];
    println!("plus long : {:?}", mots.iter().max_by_key(|m| m.len()));
    let (courts, longs): (Vec<&str>, Vec<&str>) = mots.iter().partition(|m| m.len() <= 5);
    println!("{courts:?} / {longs:?}");
}

fn collecte() {
    println!("--- collect() vers divers types ---");
    let mots = ["un", "deux", "trois"];

    let longueurs: HashMap<&str, usize> = mots.iter().map(|m| (*m, m.len())).collect();
    let mut tri: Vec<_> = longueurs.into_iter().collect();
    tri.sort();
    println!("{tri:?}");

    let phrase: String = mots
        .iter()
        .map(|m| m.to_uppercase())
        .collect::<Vec<_>>()
        .join("-");
    println!("{phrase}");

    let ok: Result<Vec<i32>, _> = "1,2,3".split(',').map(str::parse::<i32>).collect();
    let ko: Result<Vec<i32>, _> = "1,x,3".split(',').map(str::parse::<i32>).collect();
    println!("{ok:?} {}", ko.is_err());

    let somme_option: Option<i32> = [Some(1), Some(2), None].into_iter().sum();
    println!("{somme_option:?}");
}

// ───────────────────────── Implémenter Iterator ─────────────────────────

/// Il suffit de définir `next` : TOUS les adaptateurs sont alors disponibles gratuitement.
struct Fibonacci {
    courant: u64,
    suivant: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            courant: 0,
            suivant: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let resultat = self.courant;
        (self.courant, self.suivant) = (self.suivant, self.courant.checked_add(self.suivant)?);
        Some(resultat)
    }
}

fn iterateur_perso() {
    println!("--- Iterator personnalisé ---");
    let dix: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{dix:?}");
    let somme_paires: u64 = Fibonacci::new()
        .take_while(|&n| n < 4_000_000)
        .filter(|n| n % 2 == 0)
        .sum();
    println!("somme des Fibonacci pairs < 4M = {somme_paires}");
    println!(
        "nombre de termes avant dépassement de u64 : {}",
        Fibonacci::new().count()
    );
}
