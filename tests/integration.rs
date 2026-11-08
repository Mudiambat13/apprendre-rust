//! Test d'intégration : traité comme un *client externe* de la bibliothèque.
//!
//! Chaque fichier de `tests/` est compilé comme une crate séparée et ne voit que
//! l'API **publique** de `apprendre_rust`.

use apprendre_rust::texte::{compter_mots, inverser};
use apprendre_rust::{ErreurMath, additionner, diviser, est_premier};

#[test]
fn scenario_complet() {
    let somme = additionner(20, 22);
    assert_eq!(somme, 42);
    assert_eq!(diviser(somme, 2), Ok(21));
    assert!(!est_premier(21));
}

#[test]
fn les_erreurs_sont_affichables() {
    let erreur = diviser(1, 0).unwrap_err();
    assert_eq!(erreur, ErreurMath::DivisionParZero);
    assert_eq!(erreur.to_string(), "division par zéro");
}

#[test]
fn texte_roundtrip() {
    let phrase = "apprendre rust";
    assert_eq!(inverser(&inverser(phrase)), phrase);
    assert_eq!(compter_mots(phrase), 2);
}

/// Les tests peuvent renvoyer un `Result` : le `?` est alors utilisable.
#[test]
fn avec_result() -> Result<(), ErreurMath> {
    let q = diviser(100, 5)?;
    assert_eq!(q, 20);
    Ok(())
}
