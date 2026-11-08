//! # apprendre-rust
//!
//! Petite bibliothèque utilisée par le **chapitre 12 (tests)** pour illustrer :
//!
//! - les *tests unitaires* (`#[cfg(test)]` en bas de ce fichier) ;
//! - les *tests de documentation* (les blocs ```` ``` ```` ci-dessous sont compilés et exécutés
//!   par `cargo test`) ;
//! - les *tests d'intégration* (dossier `tests/` à la racine du dépôt).
//!
//! ```
//! use apprendre_rust::{additionner, diviser};
//!
//! assert_eq!(additionner(2, 3), 5);
//! assert_eq!(diviser(10, 2), Ok(5));
//! ```

use std::error::Error;
use std::fmt;

/// Erreurs mathématiques possibles.
#[derive(Debug, PartialEq, Eq)]
pub enum ErreurMath {
    /// Tentative de division par zéro.
    DivisionParZero,
    /// Le résultat dépasse la capacité d'un `i32`.
    Debordement,
}

impl fmt::Display for ErreurMath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurMath::DivisionParZero => write!(f, "division par zéro"),
            ErreurMath::Debordement => write!(f, "dépassement de capacité"),
        }
    }
}

impl Error for ErreurMath {}

/// Additionne deux entiers.
///
/// # Exemples
///
/// ```
/// use apprendre_rust::additionner;
///
/// assert_eq!(additionner(40, 2), 42);
/// assert_eq!(additionner(-1, 1), 0);
/// ```
pub fn additionner(a: i32, b: i32) -> i32 {
    a + b
}

/// Division entière qui signale proprement les cas d'erreur.
///
/// # Erreurs
///
/// - [`ErreurMath::DivisionParZero`] si `b == 0` ;
/// - [`ErreurMath::Debordement`] pour `i32::MIN / -1`.
///
/// # Exemples
///
/// ```
/// use apprendre_rust::{diviser, ErreurMath};
///
/// assert_eq!(diviser(7, 2), Ok(3));
/// assert_eq!(diviser(1, 0), Err(ErreurMath::DivisionParZero));
/// ```
pub fn diviser(a: i32, b: i32) -> Result<i32, ErreurMath> {
    if b == 0 {
        return Err(ErreurMath::DivisionParZero);
    }
    a.checked_div(b).ok_or(ErreurMath::Debordement)
}

/// Teste si un nombre est premier (algorithme naïf en `O(√n)`).
///
/// ```
/// use apprendre_rust::est_premier;
///
/// assert!(est_premier(97));
/// assert!(!est_premier(1));
/// assert!(!est_premier(100));
/// ```
pub fn est_premier(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

/// Petits utilitaires sur le texte.
pub mod texte {
    /// Inverse une chaîne en respectant les caractères Unicode.
    ///
    /// ```
    /// use apprendre_rust::texte::inverser;
    ///
    /// assert_eq!(inverser("Rust"), "tsuR");
    /// assert_eq!(inverser("été"), "été");
    /// ```
    pub fn inverser(s: &str) -> String {
        s.chars().rev().collect()
    }

    /// Compte les mots séparés par des espaces.
    ///
    /// ```
    /// use apprendre_rust::texte::compter_mots;
    ///
    /// assert_eq!(compter_mots("  un  deux trois "), 3);
    /// assert_eq!(compter_mots(""), 0);
    /// ```
    pub fn compter_mots(s: &str) -> usize {
        s.split_whitespace().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_simple() {
        assert_eq!(additionner(2, 2), 4);
    }

    #[test]
    fn division_normale() {
        assert_eq!(diviser(10, 3), Ok(3));
    }

    #[test]
    fn division_par_zero() {
        assert_eq!(diviser(10, 0), Err(ErreurMath::DivisionParZero));
    }

    #[test]
    fn division_debordement() {
        assert_eq!(diviser(i32::MIN, -1), Err(ErreurMath::Debordement));
    }

    #[test]
    fn premiers_connus() {
        let premiers = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for p in premiers {
            assert!(est_premier(p), "{p} devrait être premier");
        }
    }

    #[test]
    fn non_premiers() {
        for n in [0, 1, 4, 6, 8, 9, 10, 15, 21, 25] {
            assert!(!est_premier(n), "{n} ne devrait pas être premier");
        }
    }

    #[test]
    fn texte_inverse() {
        assert_eq!(texte::inverser("abc"), "cba");
    }
}
