//! # Chapitre 15 — Traits avancés
//!
//! Types associés, constantes associées, GAT, surcharge d'opérateurs, supertraits,
//! traits d'extension, `PhantomData` + typestate, `dyn Any`.
//!
//! Lancer : `cargo run --bin ch15_traits_avances`

#![allow(dead_code, clippy::needless_lifetimes)] // annotations explicites : but pédagogique

use std::any::Any;
use std::fmt::{self, Debug, Display};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Index, Mul, Neg, Sub};

fn main() {
    types_associes();
    gat();
    operateurs();
    supertraits();
    extension();
    typestate();
    any();
}

// ───────────── Types associés : un type « décidé » par l'implémenteur ─────────────

/// Contrairement à un paramètre générique (`Conteneur<T>`), un type associé impose UNE seule
/// implémentation par type : `Pile` ne peut avoir qu'un seul `Element`.
trait Conteneur {
    type Element;
    const NOM: &'static str; // constante associée

    fn premier(&self) -> Option<&Self::Element>;
    fn taille(&self) -> usize;

    fn est_vide(&self) -> bool {
        self.taille() == 0
    }
}

struct Pile(Vec<String>);

impl Conteneur for Pile {
    type Element = String;
    const NOM: &'static str = "pile de chaînes";

    fn premier(&self) -> Option<&String> {
        self.0.last()
    }
    fn taille(&self) -> usize {
        self.0.len()
    }
}

/// Fonction générique sur n'importe quel `Conteneur` dont l'élément est affichable.
fn decrire<C>(c: &C) -> String
where
    C: Conteneur,
    C::Element: Debug,
{
    format!(
        "{} de taille {}, premier = {:?}",
        C::NOM,
        c.taille(),
        c.premier()
    )
}

fn types_associes() {
    println!("--- Types associés ---");
    let p = Pile(vec!["a".into(), "b".into()]);
    println!("{} (vide ? {})", decrire(&p), p.est_vide());
}

// ───────────── GAT : types associés génériques (lending iterator) ─────────────

/// Un itérateur qui *prête* ses éléments (chaque `next` emprunte `self`).
/// Impossible avec `Iterator` standard, car `Item` ne peut pas dépendre du lifetime de `&mut self`.
trait ItereurPreteur {
    type Item<'a>
    where
        Self: 'a;

    fn suivant<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

/// Fenêtres glissantes MUTABLES sur une tranche (`slice::windows_mut` n'existe pas).
struct FenetresMut<'s, T> {
    tranche: &'s mut [T],
    pos: usize,
    taille: usize,
}

impl<'s, T> ItereurPreteur for FenetresMut<'s, T> {
    type Item<'a>
        = &'a mut [T]
    where
        Self: 'a;

    fn suivant<'a>(&'a mut self) -> Option<&'a mut [T]> {
        let fin = self.pos + self.taille;
        if fin > self.tranche.len() {
            return None;
        }
        let fenetre = &mut self.tranche[self.pos..fin];
        self.pos += 1;
        Some(fenetre)
    }
}

fn gat() {
    println!("--- GAT ---");
    let mut donnees = [1, 2, 3, 4, 5];
    let mut it = FenetresMut {
        tranche: &mut donnees,
        pos: 0,
        taille: 3,
    };
    while let Some(fenetre) = it.suivant() {
        fenetre[2] += fenetre[0]; // chaque fenêtre est modifiable
    }
    println!("{donnees:?}");
}

// ───────────── Surcharge d'opérateurs : traits de std::ops ─────────────

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + o.x,
            y: self.y + o.y,
        }
    }
}
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - o.x,
            y: self.y - o.y,
        }
    }
}
/// `Vec2 * f64` (scalaire). On pourrait aussi implémenter `Mul<Vec2> for f64`.
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: f64) -> Vec2 {
        Vec2 {
            x: self.x * k,
            y: self.y * k,
        }
    }
}
impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, o: Vec2) {
        self.x += o.x;
        self.y += o.y;
    }
}
impl Index<usize> for Vec2 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index {i} hors limites pour Vec2"),
        }
    }
}
impl Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        Vec2 { x, y }
    }
}

fn operateurs() {
    println!("--- Opérateurs ---");
    let a = Vec2 { x: 1.0, y: 2.0 };
    let b: Vec2 = (3.0, 4.0).into();
    let mut c = a + b * 2.0 - Vec2::default();
    c += -a;
    println!("{a} + {b}×2 − a = {c} ; c[1] = {}", c[1]);
}

// ───────────── Supertraits ─────────────

/// `Animal` exige que le type implémente aussi `Display`.
trait Animal: Display {
    fn cri(&self) -> String;
    fn presentation(&self) -> String {
        format!("Je suis {} et je fais « {} »", self, self.cri()) // possible grâce à Display
    }
}

struct Chien;
impl Display for Chien {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "un chien")
    }
}
impl Animal for Chien {
    fn cri(&self) -> String {
        "ouaf".into()
    }
}

fn supertraits() {
    println!("--- Supertraits ---");
    let animaux: Vec<Box<dyn Animal>> = vec![Box::new(Chien)];
    for a in animaux {
        println!("{}", a.presentation());
    }
}

// ───────────── Traits d'extension : ajouter des méthodes à des types existants ─────────────

trait StrExt {
    fn est_palindrome(&self) -> bool;
    fn mots_capitalises(&self) -> String;
}

impl StrExt for str {
    fn est_palindrome(&self) -> bool {
        let c: Vec<char> = self
            .chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect();
        c.iter().eq(c.iter().rev())
    }

    fn mots_capitalises(&self) -> String {
        self.split_whitespace()
            .map(|m| {
                let mut chars = m.chars();
                match chars.next() {
                    Some(premier) => premier.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn extension() {
    println!("--- Trait d'extension ---");
    println!(
        "{} {}",
        "Kayak".est_palindrome(),
        "le rust c'est chouette".mots_capitalises()
    );
}

// ───────────── Typestate : encoder l'état dans le type (PhantomData) ─────────────

struct Ouverte;
struct Fermee;
struct Verrouillee;

/// Impossible de compiler un appel invalide (ex. ouvrir une porte verrouillée).
struct Porte<Etat> {
    nom: String,
    _etat: PhantomData<Etat>, // occupe 0 octet ; relie `Etat` à la struct
}

impl Porte<Fermee> {
    fn new(nom: &str) -> Self {
        Porte {
            nom: nom.to_string(),
            _etat: PhantomData,
        }
    }
    fn ouvrir(self) -> Porte<Ouverte> {
        println!("  {} s'ouvre", self.nom);
        Porte {
            nom: self.nom,
            _etat: PhantomData,
        }
    }
    fn verrouiller(self) -> Porte<Verrouillee> {
        println!("  {} est verrouillée", self.nom);
        Porte {
            nom: self.nom,
            _etat: PhantomData,
        }
    }
}
impl Porte<Ouverte> {
    fn fermer(self) -> Porte<Fermee> {
        println!("  {} se ferme", self.nom);
        Porte {
            nom: self.nom,
            _etat: PhantomData,
        }
    }
}
impl Porte<Verrouillee> {
    fn deverrouiller(self) -> Porte<Fermee> {
        println!("  {} est déverrouillée", self.nom);
        Porte {
            nom: self.nom,
            _etat: PhantomData,
        }
    }
}

fn typestate() {
    println!("--- Typestate ---");
    let p = Porte::new("la porte d'entrée");
    let p = p.ouvrir().fermer().verrouiller().deverrouiller();
    // p.verrouiller().ouvrir(); // ← ne compile pas : `ouvrir` n'existe que sur Porte<Fermee>
    println!(
        "  taille de PhantomData : {} octet",
        size_of::<PhantomData<Ouverte>>()
    );
    let _ = p;
}

// ───────────── dyn Any : typage dynamique et downcasting ─────────────

fn decrire_inconnu(valeur: &dyn Any) -> String {
    if let Some(n) = valeur.downcast_ref::<i32>() {
        format!("un i32 : {n}")
    } else if let Some(s) = valeur.downcast_ref::<String>() {
        format!("une String : {s:?}")
    } else {
        "type inconnu".to_string()
    }
}

fn any() {
    println!("--- dyn Any ---");
    println!("{}", decrire_inconnu(&42_i32));
    println!("{}", decrire_inconnu(&String::from("salut")));
    println!("{}", decrire_inconnu(&1.5_f32));
}
