//! # Niveau 4 — Expert (chapitres 18 à 21)
//!
//! `cargo test -p exercices niveau4`
//!
//! Astuce : `cargo +nightly miri test -p exercices niveau4` détecte les comportements indéfinis.

use std::cell::RefCell;
use std::rc::Rc;

/// **Exercice 4.1 — `unsafe` encapsulé** (chapitre 18)
///
/// Découpe une tranche mutable en deux tranches mutables disjointes
/// `[0, milieu)` et `[milieu, len)`. C'est `slice::split_at_mut` : réécrivez-la avec
/// `unsafe` (pointeur brut + `std::slice::from_raw_parts_mut`).
///
/// Panique avec un message contenant `"hors limites"` si `milieu > len`.
/// Expliquez en commentaire `// SAFETY:` pourquoi votre `unsafe` est correct.
pub fn diviser_en_deux_mut<T>(tranche: &mut [T], milieu: usize) -> (&mut [T], &mut [T]) {
    // SOLUTION
    let len = tranche.len();
    assert!(milieu <= len, "milieu hors limites ({milieu} > {len})");
    let ptr = tranche.as_mut_ptr();
    // SAFETY: les deux sous-tranches [0, milieu) et [milieu, len) sont dans les bornes de
    // `tranche`, ne se chevauchent pas, et `tranche` reste empruntée en exclusivité.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, milieu),
            std::slice::from_raw_parts_mut(ptr.add(milieu), len - milieu),
        )
    }
    // FIN SOLUTION
}

/// **Exercice 4.2 — Génériques constants** (chapitre 21)
///
/// Tampon circulaire (file FIFO à capacité fixe `N`, sans allocation).
/// Quand il est plein, `pousser` ÉCRASE le plus ancien élément et le retourne.
#[derive(Debug)]
pub struct TamponCirculaire<T, const N: usize> {
    donnees: [Option<T>; N],
    debut: usize,
    longueur: usize,
}

impl<T, const N: usize> TamponCirculaire<T, N> {
    /// Crée un tampon vide. Panique si `N == 0`.
    /// Indice : `std::array::from_fn(|_| None)` initialise un `[Option<T>; N]` même si `T: !Copy`.
    pub fn new() -> Self {
        // SOLUTION
        assert!(N > 0, "la capacité doit être > 0");
        TamponCirculaire {
            donnees: std::array::from_fn(|_| None),
            debut: 0,
            longueur: 0,
        }
        // FIN SOLUTION
    }

    /// Ajoute à la fin. Si le tampon est plein, écrase le plus ancien et le retourne.
    pub fn pousser(&mut self, valeur: T) -> Option<T> {
        // SOLUTION
        if self.longueur == N {
            let ancien = self.donnees[self.debut].replace(valeur);
            self.debut = (self.debut + 1) % N;
            ancien
        } else {
            let indice = (self.debut + self.longueur) % N;
            self.donnees[indice] = Some(valeur);
            self.longueur += 1;
            None
        }
        // FIN SOLUTION
    }

    /// Retire et retourne le plus ancien élément.
    pub fn retirer(&mut self) -> Option<T> {
        // SOLUTION
        if self.longueur == 0 {
            return None;
        }
        let valeur = self.donnees[self.debut].take();
        self.debut = (self.debut + 1) % N;
        self.longueur -= 1;
        valeur
        // FIN SOLUTION
    }

    pub fn len(&self) -> usize {
        // SOLUTION
        self.longueur
        // FIN SOLUTION
    }

    pub fn is_empty(&self) -> bool {
        // SOLUTION
        self.longueur == 0
        // FIN SOLUTION
    }

    pub fn est_plein(&self) -> bool {
        // SOLUTION
        self.longueur == N
        // FIN SOLUTION
    }
}

impl<T, const N: usize> Default for TamponCirculaire<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// **Exercice 4.3 — `Drop` et ordre de destruction** (chapitres 10 et 21)
///
/// Un `Garde` écrit son nom dans un journal partagé quand il est détruit.
pub struct Garde {
    nom: &'static str,
    journal: Rc<RefCell<Vec<&'static str>>>,
}

impl Garde {
    pub fn new(nom: &'static str, journal: &Rc<RefCell<Vec<&'static str>>>) -> Garde {
        // SOLUTION
        Garde {
            nom,
            journal: Rc::clone(journal),
        }
        // FIN SOLUTION
    }
}

impl Drop for Garde {
    fn drop(&mut self) {
        // SOLUTION
        self.journal.borrow_mut().push(self.nom);
        // FIN SOLUTION
    }
}

/// Crée trois gardes `"a"`, `"b"`, `"c"` (dans cet ordre) dans un bloc, les laisse sortir de
/// portée, et retourne le contenu du journal : quel est l'ordre de destruction ?
pub fn ordre_de_destruction() -> Vec<&'static str> {
    // SOLUTION
    let journal = Rc::new(RefCell::new(Vec::new()));
    {
        let _a = Garde::new("a", &journal);
        let _b = Garde::new("b", &journal);
        let _c = Garde::new("c", &journal);
    }
    journal.borrow().clone()
    // FIN SOLUTION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn division_en_deux_tranches() {
        let mut t = [1, 2, 3, 4, 5];
        let (g, d) = diviser_en_deux_mut(&mut t, 2);
        assert_eq!(g, &[1, 2]);
        assert_eq!(d, &[3, 4, 5]);
        g[0] = 10;
        d[2] = 50;
        assert_eq!(t, [10, 2, 3, 4, 50]);
    }

    #[test]
    fn division_aux_extremites() {
        let mut t = [1, 2, 3];
        let (g, d) = diviser_en_deux_mut(&mut t, 0);
        assert!(g.is_empty());
        assert_eq!(d.len(), 3);
        let (g, d) = diviser_en_deux_mut(&mut t, 3);
        assert_eq!(g.len(), 3);
        assert!(d.is_empty());
    }

    #[test]
    fn division_avec_des_types_non_copy() {
        let mut v = vec![String::from("a"), String::from("b"), String::from("c")];
        let (g, d) = diviser_en_deux_mut(&mut v, 1);
        std::mem::swap(&mut g[0], &mut d[1]);
        assert_eq!(v, ["c", "b", "a"]);
    }

    #[test]
    #[should_panic(expected = "hors limites")]
    fn division_hors_limites() {
        let mut t = [1, 2];
        let _ = diviser_en_deux_mut(&mut t, 3);
    }

    #[test]
    fn tampon_fifo_simple() {
        let mut t: TamponCirculaire<i32, 3> = TamponCirculaire::new();
        assert!(t.is_empty());
        assert_eq!(t.pousser(1), None);
        assert_eq!(t.pousser(2), None);
        assert_eq!(t.len(), 2);
        assert_eq!(t.retirer(), Some(1));
        assert_eq!(t.retirer(), Some(2));
        assert_eq!(t.retirer(), None);
    }

    #[test]
    fn tampon_ecrase_le_plus_ancien() {
        let mut t: TamponCirculaire<&str, 3> = TamponCirculaire::new();
        for s in ["a", "b", "c"] {
            assert_eq!(t.pousser(s), None);
        }
        assert!(t.est_plein());
        assert_eq!(t.pousser("d"), Some("a")); // « a » écrasé
        assert_eq!(t.pousser("e"), Some("b"));
        assert_eq!(t.retirer(), Some("c"));
        assert_eq!(t.retirer(), Some("d"));
        assert_eq!(t.retirer(), Some("e"));
        assert!(t.is_empty());
    }

    #[test]
    fn tampon_tourne_correctement() {
        let mut t: TamponCirculaire<u32, 4> = TamponCirculaire::new();
        for tour in 0..20u32 {
            t.pousser(tour);
            if tour % 2 == 1 {
                t.retirer();
            }
        }
        // Après chaque tour impair on retire un élément : en régime établi il en reste 3,
        // et les plus anciens ont été écrasés quand le tampon était plein.
        assert_eq!(t.len(), 3);
        assert_eq!(t.retirer(), Some(17));
        assert_eq!(t.retirer(), Some(18));
        assert_eq!(t.retirer(), Some(19));
        assert_eq!(t.retirer(), None);
    }

    #[test]
    fn tampon_avec_des_types_non_copy() {
        let mut t: TamponCirculaire<String, 2> = TamponCirculaire::new();
        t.pousser("x".to_string());
        t.pousser("y".to_string());
        assert_eq!(t.pousser("z".to_string()), Some("x".to_string()));
    }

    #[test]
    fn destruction_dans_l_ordre_inverse() {
        assert_eq!(ordre_de_destruction(), vec!["c", "b", "a"]);
    }

    #[test]
    fn garde_ecrit_dans_le_journal() {
        let journal = Rc::new(RefCell::new(Vec::new()));
        {
            let _g = Garde::new("seul", &journal);
            assert!(journal.borrow().is_empty());
        }
        assert_eq!(*journal.borrow(), vec!["seul"]);
    }
}
