//! # Chapitre 7 — Génériques et traits
//!
//! Lancer : `cargo run --bin ch07_traits_generiques`

use std::collections::HashMap;
use std::fmt::{self, Debug, Display};
use std::hash::Hash;

fn main() {
    generiques();
    traits();
    dispatch();
    implementations_generales();
}

// ───────────────────────── Génériques ─────────────────────────

/// `T: PartialOrd` : « T doit pouvoir être comparé ». Monomorphisation → zéro coût à l'exécution.
fn plus_grand<T: PartialOrd>(liste: &[T]) -> Option<&T> {
    let mut iter = liste.iter();
    let mut max = iter.next()?;
    for x in iter {
        if x > max {
            max = x;
        }
    }
    Some(max)
}

#[derive(Debug, Clone, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Méthodes disponibles uniquement pour un type précis.
impl Point<f64> {
    fn distance_origine(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// `where` rend les contraintes complexes lisibles.
fn compter_occurrences<T>(elements: &[T]) -> HashMap<&T, usize>
where
    T: Hash + Eq,
{
    let mut m = HashMap::new();
    for e in elements {
        *m.entry(e).or_insert(0) += 1;
    }
    m
}

fn generiques() {
    println!("--- Génériques ---");
    println!(
        "{:?} {:?} {:?}",
        plus_grand(&[3, 9, 2]),
        plus_grand(&["a", "z", "m"]),
        plus_grand::<f64>(&[])
    );
    let p = Point::new(3.0, 4.0);
    println!("{p:?} → distance = {}", p.distance_origine());
    let mut occ: Vec<_> = compter_occurrences(&["a", "b", "a"]).into_iter().collect();
    occ.sort();
    println!("{occ:?}");
}

// ───────────────────────── Traits ─────────────────────────

/// Un trait définit un comportement partagé. Il peut fournir des implémentations par défaut.
trait Resumable {
    fn auteur(&self) -> String;

    fn resume(&self) -> String {
        format!("(Lire plus de {}…)", self.auteur())
    }
}

struct Article {
    titre: String,
    auteur: String,
}

struct Tweet {
    pseudo: String,
    contenu: String,
}

impl Resumable for Article {
    fn auteur(&self) -> String {
        self.auteur.clone()
    }
    fn resume(&self) -> String {
        format!("{}, par {}", self.titre, self.auteur)
    }
}

impl Resumable for Tweet {
    fn auteur(&self) -> String {
        format!("@{}", self.pseudo)
    }
    // `resume` : implémentation par défaut conservée.
}

/// `impl Trait` en argument : sucre pour un générique `<T: Resumable>`.
fn notifier(element: &impl Resumable) {
    println!("Nouvelle publication ! {}", element.resume());
}

/// `impl Trait` en retour : on cache le type concret.
fn creer_resumable(court: bool) -> impl Display {
    if court {
        "court".to_string()
    } else {
        "un peu plus long".to_string()
    }
}

/// Implémenter les traits standard permet d'utiliser `println!`, `==`, `+`, etc.
struct Fraction {
    num: i32,
    den: i32,
}

impl Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

fn traits() {
    println!("--- Traits ---");
    let a = Article {
        titre: "Rust 2024".into(),
        auteur: "Ferris".into(),
    };
    let t = Tweet {
        pseudo: "rustlang".into(),
        contenu: "Edition 2024 !".into(),
    };
    notifier(&a);
    notifier(&t);
    println!("{} | {}", t.contenu, creer_resumable(true));
    println!("{}", Fraction { num: 3, den: 4 });
}

// ───────────────────────── Dispatch statique vs dynamique ─────────────────────────

trait Forme {
    fn aire(&self) -> f64;
    fn nom(&self) -> &'static str;
}

struct Cercle(f64);
struct Carre(f64);

impl Forme for Cercle {
    fn aire(&self) -> f64 {
        std::f64::consts::PI * self.0 * self.0
    }
    fn nom(&self) -> &'static str {
        "cercle"
    }
}
impl Forme for Carre {
    fn aire(&self) -> f64 {
        self.0 * self.0
    }
    fn nom(&self) -> &'static str {
        "carré"
    }
}

/// Statique : une copie du code par type concret, appels inlinables. Rapide mais gonfle le binaire.
fn aire_statique<F: Forme>(f: &F) -> f64 {
    f.aire()
}

/// Dynamique : `dyn Forme` = pointeur + vtable. Permet des collections hétérogènes.
fn aire_totale(formes: &[Box<dyn Forme>]) -> f64 {
    formes.iter().map(|f| f.aire()).sum()
}

fn dispatch() {
    println!("--- Statique vs dynamique ---");
    println!("{:.2}", aire_statique(&Cercle(1.0)));
    let formes: Vec<Box<dyn Forme>> = vec![Box::new(Cercle(2.0)), Box::new(Carre(3.0))];
    for f in &formes {
        println!("  {} → {:.2}", f.nom(), f.aire());
    }
    println!("aire totale = {:.2}", aire_totale(&formes));
}

// ───────────────────────── Blanket implementations ─────────────────────────

trait Decrire {
    fn decrire(&self) -> String;
}

/// « Pour TOUT type qui implémente Debug, on implémente Decrire. » (comme `ToString` pour `Display`)
impl<T: Debug> Decrire for T {
    fn decrire(&self) -> String {
        format!("<<{self:?}>>")
    }
}

fn implementations_generales() {
    println!("--- Blanket impl ---");
    println!(
        "{} {} {}",
        42.decrire(),
        "salut".decrire(),
        vec![1, 2].decrire()
    );
}
