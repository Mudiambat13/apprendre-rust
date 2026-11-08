//! # Chapitre 20 — Macros procédurales
//!
//! Une macro procédurale est une **fonction Rust** qui reçoit un flux de jetons (`TokenStream`)
//! et en renvoie un autre. Elle doit vivre dans une crate dédiée (`proc-macro = true`) :
//! voir `crates/macros_demo/src/lib.rs` (à lire en premier).
//!
//! Lancer : `cargo run --bin ch20_proc_macros`
//! Voir le code généré (nécessite `cargo-expand`) : `cargo expand --bin ch20_proc_macros`

use macros_demo::{Describe, carre_const, chronometre};
use std::collections::HashMap;

#[derive(Describe)]
#[allow(dead_code)]
struct Personne {
    nom: String,
    age: u32,
    pub amis: Vec<String>,
    scores: HashMap<String, u32>, // la virgule dans les chevrons est bien gérée
}

#[derive(Describe)]
struct Vide {}

/// La signature est intacte ; le corps est enveloppé pour mesurer le temps.
#[chronometre]
fn calcul_lent(n: u64) -> u64 {
    if n == 0 {
        return 0; // `return` fonctionne grâce à la closure
    }
    (1..=n).map(|x| x * x % 7).sum()
}

#[chronometre]
fn lecture(texte: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = texte.parse()?; // `?` fonctionne aussi
    Ok(n * 2)
}

fn main() {
    println!("--- #[derive(Describe)] ---");
    println!("{}", Personne::describe());
    println!("champs : {:?}", Personne::noms_des_champs());
    println!("{}", Vide::describe());

    println!("--- #[chronometre] (voir stderr) ---");
    println!("résultat = {}", calcul_lent(5_000_000));
    println!("{:?} {:?}", lecture("21"), lecture("x").is_err());

    println!("--- carre_const!(…) ---");
    const CARRE: i64 = carre_const!(12); // calculé à la compilation, utilisable dans un `const`
    println!("12² = {CARRE}");
}
