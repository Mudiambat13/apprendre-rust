//! # Chapitre 4 — Structs, enums et pattern matching
//!
//! Lancer : `cargo run --bin ch04_structs_enums`

#![allow(dead_code)]

fn main() {
    structs();
    enums();
    option();
}

// ───────────────────────────── Structs ─────────────────────────────

/// Struct « classique » avec champs nommés. `derive` génère des implémentations de traits.
#[derive(Debug, Clone, PartialEq, Default)]
struct Utilisateur {
    nom: String,
    email: String,
    actif: bool,
    connexions: u32,
}

/// Struct-tuple : utile pour créer des types distincts (pattern « newtype »).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Metres(f64);

/// Struct « unité » : aucun champ, sert de marqueur.
struct Marqueur;

#[derive(Debug)]
struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

impl Rectangle {
    /// Fonction associée (pas de `self`) : appelée avec `Rectangle::carre(3.0)`.
    fn carre(cote: f64) -> Self {
        Self {
            largeur: cote,
            hauteur: cote,
        }
    }

    /// Méthode : le premier paramètre est `self`, `&self` ou `&mut self`.
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }

    fn agrandir(&mut self, facteur: f64) {
        self.largeur *= facteur;
        self.hauteur *= facteur;
    }

    fn peut_contenir(&self, autre: &Rectangle) -> bool {
        self.largeur >= autre.largeur && self.hauteur >= autre.hauteur
    }
}

fn structs() {
    println!("--- Structs ---");
    let alice = Utilisateur {
        nom: String::from("Alice"),
        email: String::from("alice@example.com"),
        actif: true,
        connexions: 1,
    };
    // Syntaxe de mise à jour : reprend les autres champs depuis une autre instance.
    let bob = Utilisateur {
        nom: "Bob".into(),
        email: "bob@example.com".into(),
        ..alice.clone()
    };
    let vide = Utilisateur::default();
    println!("{alice:?}\n{bob:#?}\n{vide:?}");

    let distance = Metres(12.5);
    println!("{distance:?} → {}", distance.0);

    let mut r = Rectangle {
        largeur: 4.0,
        hauteur: 3.0,
    };
    println!("aire = {}", r.aire());
    r.agrandir(2.0);
    let petit = Rectangle::carre(5.0);
    println!("{r:?} contient {petit:?} ? {}", r.peut_contenir(&petit));
}

// ───────────────────────────── Enums ─────────────────────────────

/// Les variantes d'un enum peuvent porter des données de types différents :
/// c'est un *type somme* (algebraic data type).
#[derive(Debug)]
enum Message {
    Quitter,                     // aucune donnée
    Deplacer { x: i32, y: i32 }, // champs nommés
    Ecrire(String),              // un String
    Couleur(u8, u8, u8),         // un tuple
}

impl Message {
    fn traiter(&self) -> String {
        match self {
            Message::Quitter => "au revoir".to_string(),
            Message::Deplacer { x, y } => format!("déplacement vers ({x}, {y})"),
            Message::Ecrire(texte) => format!("texte : {texte}"),
            Message::Couleur(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Jour {
    Lundi = 1,
    Mardi,
    Mercredi,
}

fn enums() {
    println!("--- Enums ---");
    let messages = [
        Message::Quitter,
        Message::Deplacer { x: 10, y: -4 },
        Message::Ecrire("salut".into()),
        Message::Couleur(255, 128, 0),
    ];
    for m in &messages {
        println!("{m:?} → {}", m.traiter());
    }
    println!(
        "{:?} vaut {} ; Mardi > Lundi ? {}",
        Jour::Mercredi,
        Jour::Mercredi as i32,
        Jour::Mardi > Jour::Lundi
    );
}

// ───────────────────────────── Option ─────────────────────────────

/// Rust n'a pas de `null`. L'absence de valeur est un type : `Option<T>`
///   enum Option<T> { None, Some(T) }
fn diviser(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn option() {
    println!("--- Option ---");
    match diviser(10.0, 4.0) {
        Some(q) => println!("quotient = {q}"),
        None => println!("division impossible"),
    }

    // Combinateurs : évitent des `match` verbeux.
    let doublé = diviser(9.0, 3.0).map(|q| q * 2.0);
    let defaut = diviser(1.0, 0.0).unwrap_or(f64::NAN);
    println!("{doublé:?} {defaut}");

    let prenom: Option<&str> = Some("alice");
    let majuscule = prenom.map(|p| p.to_uppercase()).unwrap_or_default();
    println!("{majuscule}");

    // `?` fonctionne aussi avec Option dans une fonction qui renvoie Option.
    fn deux_tiers(a: f64) -> Option<f64> {
        let tiers = diviser(a, 3.0)?;
        Some(tiers * 2.0)
    }
    println!("{:?}", deux_tiers(9.0));
}
