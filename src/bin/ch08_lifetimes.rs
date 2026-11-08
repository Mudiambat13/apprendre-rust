//! # Chapitre 8 — Durées de vie (lifetimes)
//!
//! Un lifetime n'est PAS une durée mesurée en secondes : c'est une **annotation** qui dit au
//! compilateur « cette référence reste valide au moins aussi longtemps que celle-là ».
//! Les lifetimes ne changent jamais la durée de vie réelle d'une valeur : ils la *décrivent*.
//!
//! Lancer : `cargo run --bin ch08_lifetimes`

#![allow(dead_code, clippy::needless_lifetimes)] // annotations explicites : but pédagogique

fn main() {
    println!("--- Fonctions ---");
    let a = String::from("longue phrase");
    let resultat;
    {
        let b = String::from("court");
        resultat = plus_long(a.as_str(), b.as_str());
        println!("dans le bloc : {resultat}");
    }
    // println!("{resultat}"); // ← ERREUR si on l'utilise ici : `b` n'existe plus,
    //                        //   et 'a est le MINIMUM des durées de vie de a et b.

    println!("--- Structs avec références ---");
    let roman = String::from("Appelez-moi Ishmael. Il y a quelques années…");
    let premiere_phrase = roman.split('.').next().expect("au moins un morceau");
    let extrait = Extrait {
        partie: premiere_phrase,
    };
    println!("{} (niveau {})", extrait.partie, extrait.niveau());
    println!("{}", extrait.annonce("Attention"));

    println!("--- Itérateur emprunteur ---");
    let texte = String::from("un deux trois");
    let mots = Mots::new(&texte);
    println!("{:?}", mots.collect::<Vec<_>>());

    println!("--- 'static ---");
    let s: &'static str = "stocké dans le binaire, valide pour tout le programme";
    println!("{s}");
    // `T: 'static` ne veut PAS dire « vit éternellement » mais « ne contient aucune référence
    // à durée limitée » : un `String` possédé est 'static, pas un `&'a str`.
    afficher_static(String::from("possédé donc 'static"));
}

/// `'a` : la référence retournée vit AU PLUS aussi longtemps que les deux arguments.
fn plus_long<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

/// Ici le retour ne dépend que de `x` : `y` peut avoir un lifetime indépendant.
fn premier<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

// ───────────── Règles d'élision : quand on peut omettre les annotations ─────────────
//
// 1. Chaque référence en entrée reçoit son propre lifetime.
// 2. S'il y a exactement UN lifetime en entrée, il est donné à toutes les sorties.
// 3. S'il y a `&self` / `&mut self`, son lifetime est donné à toutes les sorties.
//
// Donc ceci compile sans annotation :
fn premier_mot(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
// … équivalent à : fn premier_mot<'a>(s: &'a str) -> &'a str

// Mais pour `plus_long`, deux entrées et pas de self : la règle 2 ne s'applique pas → annotation requise.

/// Une struct qui contient une référence doit annoncer son lifetime.
/// L'`Extrait` ne peut pas survivre au texte qu'il référence.
struct Extrait<'a> {
    partie: &'a str,
}

impl<'a> Extrait<'a> {
    // Règle 3 : le retour prend le lifetime de &self.
    fn niveau(&self) -> usize {
        3
    }

    fn annonce(&self, message: &str) -> String {
        format!("{message} : {}", self.partie)
    }

    /// Ici on retourne une référence liée au *texte d'origine* ('a), pas à `&self` :
    /// le résultat peut donc survivre à l'`Extrait` lui-même.
    fn partie(&self) -> &'a str {
        self.partie
    }
}

/// Un itérateur qui emprunte la chaîne d'origine.
struct Mots<'t> {
    reste: &'t str,
}

impl<'t> Mots<'t> {
    fn new(texte: &'t str) -> Self {
        Mots { reste: texte }
    }
}

impl<'t> Iterator for Mots<'t> {
    type Item = &'t str;

    fn next(&mut self) -> Option<Self::Item> {
        let sans_espaces = self.reste.trim_start();
        if sans_espaces.is_empty() {
            return None;
        }
        let fin = sans_espaces.find(' ').unwrap_or(sans_espaces.len());
        let (mot, suite) = sans_espaces.split_at(fin);
        self.reste = suite;
        Some(mot)
    }
}

fn afficher_static<T: std::fmt::Display + 'static>(valeur: T) {
    println!("{valeur}");
}

// ───────────── Pour aller plus loin ─────────────
// * Bornes de lifetime : `T: 'a`, `'b: 'a` (« 'b dure au moins autant que 'a »).
// * HRTB : `for<'a> Fn(&'a str) -> &'a str` (voir chapitre 21).
// * Variance : `&'long T` peut être utilisé là où `&'short T` est attendu (voir Nomicon).
