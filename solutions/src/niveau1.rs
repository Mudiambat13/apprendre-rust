//! # Niveau 1 — Fondations (chapitres 1 à 6)
//!
//! Remplacez chaque `todo!()` par votre implémentation, puis lancez :
//! `cargo test -p exercices niveau1`

use std::collections::HashMap;
use std::num::ParseIntError;

/// **Exercice 1.1 — FizzBuzz** (chapitre 2)
///
/// Retourne `"Fizz"` si `n` est multiple de 3, `"Buzz"` s'il est multiple de 5,
/// `"FizzBuzz"` s'il est multiple des deux, sinon `n` écrit en texte.
pub fn fizzbuzz(n: u32) -> String {
    // SOLUTION
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
    // FIN SOLUTION
}

/// **Exercice 1.2 — Palindrome** (chapitres 3 et 5)
///
/// Vrai si le texte se lit pareil dans les deux sens, en ignorant la casse et tout ce qui
/// n'est pas alphanumérique. Indice : `char::is_alphanumeric`, `to_lowercase`.
pub fn est_palindrome(texte: &str) -> bool {
    // SOLUTION
    let nettoye: Vec<char> = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    nettoye.iter().eq(nettoye.iter().rev())
    // FIN SOLUTION
}

/// **Exercice 1.3 — Voyelles** (chapitre 5)
///
/// Compte les voyelles `a e i o u` (minuscules ou majuscules, sans accents).
pub fn compter_voyelles(texte: &str) -> usize {
    // SOLUTION
    texte.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
    // FIN SOLUTION
}

/// **Exercice 1.4 — Somme d'une liste CSV** (chapitre 6)
///
/// `"1, 2, 3"` → `Ok(6)`. Si un élément n'est pas un entier : `Err(ParseIntError)`.
/// Indice : `split`, `trim`, `parse::<i64>()`, et le fait qu'un itérateur de `Result`
/// peut être sommé directement en un `Result`.
pub fn somme_csv(texte: &str) -> Result<i64, ParseIntError> {
    // SOLUTION
    texte
        .split(',')
        .map(|morceau| morceau.trim().parse::<i64>())
        .sum()
    // FIN SOLUTION
}

/// **Exercice 1.5 — Fréquence des mots** (chapitre 5)
///
/// Retourne les mots (en minuscules, séparés par tout caractère non alphanumérique) avec leur
/// nombre d'occurrences, triés par nombre décroissant puis par ordre alphabétique.
///
/// `"le chat et le chien"` → `[("le", 2), ("chat", 1), ("chien", 1), ("et", 1)]`
pub fn frequences_mots(texte: &str) -> Vec<(String, usize)> {
    // SOLUTION
    let mut compteur: HashMap<String, usize> = HashMap::new();
    for mot in texte
        .split(|c: char| !c.is_alphanumeric())
        .filter(|m| !m.is_empty())
    {
        *compteur.entry(mot.to_lowercase()).or_insert(0) += 1;
    }
    let mut resultat: Vec<(String, usize)> = compteur.into_iter().collect();
    resultat.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    resultat
    // FIN SOLUTION
}

/// **Exercice 1.6 — Une struct avec des méthodes** (chapitre 4)
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub largeur: f64,
    pub hauteur: f64,
}

impl Rectangle {
    /// Surface du rectangle.
    pub fn aire(&self) -> f64 {
        // SOLUTION
        self.largeur * self.hauteur
        // FIN SOLUTION
    }

    /// Vrai si largeur et hauteur sont égales.
    pub fn est_carre(&self) -> bool {
        // SOLUTION
        (self.largeur - self.hauteur).abs() < f64::EPSILON
        // FIN SOLUTION
    }

    /// Vrai si `autre` peut être placé entièrement dans `self` (sans rotation).
    pub fn peut_contenir(&self, autre: &Rectangle) -> bool {
        // SOLUTION
        self.largeur >= autre.largeur && self.hauteur >= autre.hauteur
        // FIN SOLUTION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_cas_de_base() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(14), "14");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
    }

    #[test]
    fn palindromes() {
        assert!(est_palindrome(""));
        assert!(est_palindrome("radar"));
        assert!(est_palindrome("Esope reste ici et se repose"));
        assert!(est_palindrome("A man, a plan, a canal: Panama"));
        assert!(!est_palindrome("Rust"));
        assert!(!est_palindrome("presque"));
    }

    #[test]
    fn voyelles() {
        assert_eq!(compter_voyelles(""), 0);
        assert_eq!(compter_voyelles("Rust"), 1);
        assert_eq!(compter_voyelles("Bonjour"), 3);
        assert_eq!(compter_voyelles("AEIOU aeiou"), 10);
        assert_eq!(compter_voyelles("rhythm"), 0);
    }

    #[test]
    fn somme_csv_valide() {
        assert_eq!(somme_csv("1, 2, 3"), Ok(6));
        assert_eq!(somme_csv("10"), Ok(10));
        assert_eq!(somme_csv("-5,5, 100"), Ok(100));
    }

    #[test]
    fn somme_csv_invalide() {
        assert!(somme_csv("1, deux, 3").is_err());
        assert!(somme_csv("1,,3").is_err());
    }

    #[test]
    fn frequences() {
        let f = frequences_mots("le chat et le chien et le rat");
        assert_eq!(
            f,
            vec![
                ("le".to_string(), 3),
                ("et".to_string(), 2),
                ("chat".to_string(), 1),
                ("chien".to_string(), 1),
                ("rat".to_string(), 1),
            ]
        );
        assert_eq!(
            frequences_mots("Rust, rust! RUST?"),
            vec![("rust".to_string(), 3)]
        );
        assert!(frequences_mots("  ...  ").is_empty());
    }

    #[test]
    fn rectangle() {
        let r = Rectangle {
            largeur: 4.0,
            hauteur: 3.0,
        };
        assert_eq!(r.aire(), 12.0);
        assert!(!r.est_carre());
        assert!(
            Rectangle {
                largeur: 2.0,
                hauteur: 2.0
            }
            .est_carre()
        );
        assert!(r.peut_contenir(&Rectangle {
            largeur: 4.0,
            hauteur: 1.0
        }));
        assert!(!r.peut_contenir(&Rectangle {
            largeur: 5.0,
            hauteur: 1.0
        }));
    }
}
