//! # Chapitre 12 — Tester son code
//!
//! Trois niveaux de tests dans Rust :
//!   1. **Tests unitaires** : dans le même fichier, module `#[cfg(test)]` (accès aux éléments privés).
//!   2. **Tests d'intégration** : dossier `tests/` (API publique uniquement) — voir `tests/integration.rs`.
//!   3. **Tests de documentation** : exemples dans les commentaires `///` — voir `src/lib.rs`.
//!
//! Lancer les tests de ce chapitre :
//! ```text
//! cargo test --bin ch12_tests
//! cargo test --bin ch12_tests -- --nocapture      # afficher les println!
//! cargo test --bin ch12_tests palindrome          # filtrer par nom
//! cargo test --bin ch12_tests -- --ignored        # lancer les tests ignorés
//! ```

fn main() {
    println!("Ce chapitre se lit avec : cargo test --bin ch12_tests");
    println!("10 → premier ? {}", est_premier(10));
    println!(
        "'Esope reste ici et se repose' palindrome ? {}",
        est_palindrome("Esope reste ici et se repose")
    );
}

pub fn est_premier(n: u32) -> bool {
    n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

pub fn est_palindrome(texte: &str) -> bool {
    let nettoye: Vec<char> = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    nettoye.iter().eq(nettoye.iter().rev())
}

pub fn diviser(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division par zéro !");
    }
    a / b
}

#[derive(Debug, PartialEq)]
pub struct Compte {
    solde: i64,
}

impl Compte {
    pub fn new() -> Self {
        Compte { solde: 0 }
    }

    pub fn deposer(&mut self, montant: i64) -> Result<(), String> {
        if montant <= 0 {
            return Err(format!("montant invalide : {montant}"));
        }
        self.solde += montant;
        Ok(())
    }

    pub fn retirer(&mut self, montant: i64) -> Result<(), String> {
        if montant > self.solde {
            return Err("solde insuffisant".to_string());
        }
        self.solde -= montant;
        Ok(())
    }

    #[allow(dead_code)] // utilisé uniquement par les tests
    fn solde_brut(&self) -> i64 {
        self.solde // privé : testable depuis le module `tests` enfant
    }
}

impl Default for Compte {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // `super::*` importe tout le module parent, y compris les éléments privés.
    use super::*;

    // Fonction utilitaire de test (fixture) : factorise la mise en place.
    fn compte_avec(solde: i64) -> Compte {
        let mut c = Compte::new();
        c.deposer(solde).unwrap();
        c
    }

    #[test]
    fn premiers_nombres() {
        assert!(est_premier(2));
        assert!(est_premier(97));
        assert!(!est_premier(0));
        assert!(!est_premier(1));
        assert!(!est_premier(91)); // 7 × 13
    }

    #[test]
    fn palindromes() {
        assert!(est_palindrome("Esope reste ici et se repose"));
        assert!(est_palindrome("Radar"));
        assert!(!est_palindrome("Rust"));
    }

    #[test]
    fn depot_et_retrait() {
        let mut c = compte_avec(100);
        assert_eq!(c.retirer(30), Ok(()));
        assert_eq!(c.solde_brut(), 70); // accès à une méthode privée
        assert_ne!(c, Compte::new());
    }

    #[test]
    fn retrait_trop_grand() {
        let mut c = compte_avec(10);
        let err = c.retirer(50).unwrap_err();
        assert_eq!(err, "solde insuffisant");
        assert_eq!(
            c.solde_brut(),
            10,
            "le solde ne doit pas changer après un échec"
        );
    }

    // Tester qu'une fonction panique, avec (optionnellement) un fragment du message attendu.
    #[test]
    #[should_panic(expected = "division par zéro")]
    fn division_par_zero_panique() {
        diviser(1, 0);
    }

    // Un test peut renvoyer Result : on peut alors utiliser `?`.
    #[test]
    fn avec_le_point_d_interrogation() -> Result<(), String> {
        let mut c = Compte::new();
        c.deposer(5)?;
        c.deposer(5)?;
        assert_eq!(c.solde_brut(), 10);
        Ok(())
    }

    #[test]
    #[ignore = "exemple de test lent, lancé avec --ignored"]
    fn test_lent() {
        let somme: u64 = (1..=50_000_000u64).sum();
        assert_eq!(somme, 1_250_000_025_000_000);
    }

    // Assertions pour les flottants : ne JAMAIS comparer avec `==`.
    #[test]
    fn flottants() {
        let somme = 0.1 + 0.2;
        assert!((somme - 0.3_f64).abs() < 1e-9);
        assert_ne!(somme, 0.3); // surprise classique !
    }
}
