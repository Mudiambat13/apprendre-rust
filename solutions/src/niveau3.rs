//! # Niveau 3 — Avancé (chapitres 13 à 17)
//!
//! `cargo test -p exercices niveau3`

use std::collections::HashMap;
use std::ops::{Add, Mul, Neg, Sub};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// **Exercice 3.1 — Somme parallèle** (chapitre 13)
///
/// Découpe `donnees` en `nb_threads` morceaux (le dernier peut être plus petit), somme chaque
/// morceau dans son propre thread, puis additionne les sommes partielles.
///
/// - `nb_threads == 0` est traité comme `1` ;
/// - `donnees` vide → `0`.
///
/// Indice : `thread::scope` permet d'emprunter `donnees` sans la copier ni utiliser `Arc`.
pub fn somme_parallele(donnees: &[u64], nb_threads: usize) -> u64 {
    // SOLUTION
    if donnees.is_empty() {
        return 0;
    }
    let nb = nb_threads.max(1);
    let taille_morceau = donnees.len().div_ceil(nb);
    thread::scope(|s| {
        let handles: Vec<_> = donnees
            .chunks(taille_morceau)
            .map(|morceau| s.spawn(move || morceau.iter().sum::<u64>()))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("thread paniqué"))
            .sum()
    })
    // FIN SOLUTION
}

/// **Exercice 3.2 — Surcharge d'opérateurs** (chapitre 15)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vecteur2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Vecteur2 {
    type Output = Vecteur2;
    fn add(self, autre: Vecteur2) -> Vecteur2 {
        // SOLUTION
        Vecteur2 {
            x: self.x + autre.x,
            y: self.y + autre.y,
        }
        // FIN SOLUTION
    }
}

impl Sub for Vecteur2 {
    type Output = Vecteur2;
    fn sub(self, autre: Vecteur2) -> Vecteur2 {
        // SOLUTION
        Vecteur2 {
            x: self.x - autre.x,
            y: self.y - autre.y,
        }
        // FIN SOLUTION
    }
}

/// Multiplication par un scalaire : `Vecteur2 * f64`.
impl Mul<f64> for Vecteur2 {
    type Output = Vecteur2;
    fn mul(self, k: f64) -> Vecteur2 {
        // SOLUTION
        Vecteur2 {
            x: self.x * k,
            y: self.y * k,
        }
        // FIN SOLUTION
    }
}

impl Neg for Vecteur2 {
    type Output = Vecteur2;
    fn neg(self) -> Vecteur2 {
        // SOLUTION
        Vecteur2 {
            x: -self.x,
            y: -self.y,
        }
        // FIN SOLUTION
    }
}

impl Vecteur2 {
    /// Produit scalaire.
    pub fn produit_scalaire(&self, autre: &Vecteur2) -> f64 {
        // SOLUTION
        self.x * autre.x + self.y * autre.y
        // FIN SOLUTION
    }

    /// Norme euclidienne.
    pub fn norme(&self) -> f64 {
        // SOLUTION
        self.produit_scalaire(self).sqrt()
        // FIN SOLUTION
    }
}

/// **Exercice 3.3 — Atomiques** (chapitre 13)
///
/// Un compteur partageable entre threads **sans `Mutex`** (utilisez `AtomicUsize`).
#[derive(Debug, Default)]
pub struct Compteur {
    valeur: AtomicUsize,
}

impl Compteur {
    pub fn new() -> Self {
        // SOLUTION
        Compteur {
            valeur: AtomicUsize::new(0),
        }
        // FIN SOLUTION
    }

    /// Ajoute 1 ; prend `&self` (pas `&mut self`) : c'est le but des atomiques.
    pub fn incrementer(&self) {
        // SOLUTION
        self.valeur.fetch_add(1, Ordering::Relaxed);
        // FIN SOLUTION
    }

    pub fn valeur(&self) -> usize {
        // SOLUTION
        self.valeur.load(Ordering::SeqCst)
        // FIN SOLUTION
    }
}

/// **Exercice 3.4 — Map/reduce parallèle** (chapitres 5 et 13)
///
/// Compte les mots de chaque texte dans un thread séparé (en minuscules, séparés par tout
/// caractère non alphanumérique), puis fusionne les tables en une seule.
pub fn compter_mots_parallele(textes: Vec<String>) -> HashMap<String, usize> {
    // SOLUTION
    let handles: Vec<_> = textes
        .into_iter()
        .map(|texte| {
            thread::spawn(move || {
                let mut local: HashMap<String, usize> = HashMap::new();
                for mot in texte
                    .split(|c: char| !c.is_alphanumeric())
                    .filter(|m| !m.is_empty())
                {
                    *local.entry(mot.to_lowercase()).or_insert(0) += 1;
                }
                local
            })
        })
        .collect();

    let mut total: HashMap<String, usize> = HashMap::new();
    for h in handles {
        for (mot, n) in h.join().expect("thread paniqué") {
            *total.entry(mot).or_insert(0) += n;
        }
    }
    total
    // FIN SOLUTION
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn somme_parallele_correcte() {
        let donnees: Vec<u64> = (1..=1000).collect();
        for nb in [1, 2, 3, 7, 16, 1000, 5000] {
            assert_eq!(somme_parallele(&donnees, nb), 500_500, "avec {nb} threads");
        }
    }

    #[test]
    fn somme_parallele_cas_limites() {
        assert_eq!(somme_parallele(&[], 4), 0);
        assert_eq!(somme_parallele(&[42], 4), 42);
        assert_eq!(somme_parallele(&[1, 2, 3], 0), 6);
    }

    #[test]
    fn vecteurs() {
        let a = Vecteur2 { x: 1.0, y: 2.0 };
        let b = Vecteur2 { x: 3.0, y: 4.0 };
        assert_eq!(a + b, Vecteur2 { x: 4.0, y: 6.0 });
        assert_eq!(b - a, Vecteur2 { x: 2.0, y: 2.0 });
        assert_eq!(a * 3.0, Vecteur2 { x: 3.0, y: 6.0 });
        assert_eq!(-a, Vecteur2 { x: -1.0, y: -2.0 });
        assert_eq!(a.produit_scalaire(&b), 11.0);
        assert_eq!(b.norme(), 5.0);
    }

    #[test]
    fn compteur_partage_entre_threads() {
        let compteur = Arc::new(Compteur::new());
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let c = Arc::clone(&compteur);
                thread::spawn(move || {
                    for _ in 0..1000 {
                        c.incrementer();
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(compteur.valeur(), 8000);
    }

    #[test]
    fn mots_en_parallele() {
        let textes = vec![
            "le chat et le chien".to_string(),
            "Le rat, LE rat !".to_string(),
            String::new(),
        ];
        let r = compter_mots_parallele(textes);
        assert_eq!(r["le"], 4);
        assert_eq!(r["rat"], 2);
        assert_eq!(r["chat"], 1);
        assert_eq!(r.len(), 5); // le, chat, et, chien, rat
        assert!(compter_mots_parallele(vec![]).is_empty());
    }
}
