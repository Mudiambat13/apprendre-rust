//! # Niveau 2 — Intermédiaire (chapitres 7 à 12)
//!
//! `cargo test -p exercices niveau2`

/// **Exercice 2.1 — Lifetimes** (chapitre 8)
///
/// Retourne la chaîne la plus longue ; en cas d'égalité, la première.
/// La signature (avec ses lifetimes) est fournie : expliquez-vous pourquoi `'a` est nécessaire ?
pub fn plus_long<'a>(a: &'a str, b: &'a str) -> &'a str {
    // SOLUTION
    if b.len() > a.len() { b } else { a }
    // FIN SOLUTION
}

/// **Exercice 2.2 — Une pile générique** (chapitre 7)
#[derive(Debug, Default)]
pub struct Pile<T> {
    elements: Vec<T>,
}

impl<T> Pile<T> {
    pub fn new() -> Self {
        // SOLUTION
        Pile {
            elements: Vec::new(),
        }
        // FIN SOLUTION
    }

    /// Ajoute au sommet.
    pub fn empiler(&mut self, valeur: T) {
        // SOLUTION
        self.elements.push(valeur);
        // FIN SOLUTION
    }

    /// Retire et retourne le sommet (`None` si vide).
    pub fn depiler(&mut self) -> Option<T> {
        // SOLUTION
        self.elements.pop()
        // FIN SOLUTION
    }

    /// Regarde le sommet sans le retirer.
    pub fn sommet(&self) -> Option<&T> {
        // SOLUTION
        self.elements.last()
        // FIN SOLUTION
    }

    pub fn taille(&self) -> usize {
        // SOLUTION
        self.elements.len()
        // FIN SOLUTION
    }

    pub fn est_vide(&self) -> bool {
        // SOLUTION
        self.elements.is_empty()
        // FIN SOLUTION
    }
}

/// **Exercice 2.3 — Composition de fonctions** (chapitre 9)
///
/// `composer(f, g)(x)` vaut `g(f(x))`.
pub fn composer<A, B, C>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
) -> Box<dyn Fn(A) -> C>
where
    A: 'static,
    B: 'static,
    C: 'static,
{
    // SOLUTION
    Box::new(move |x| g(f(x)))
    // FIN SOLUTION
}

/// **Exercice 2.4 — Itérateurs** (chapitre 9)
///
/// Somme des carrés des nombres PAIRS de la tranche, en une seule chaîne d'itérateurs.
pub fn somme_carres_pairs(valeurs: &[i32]) -> i32 {
    // SOLUTION
    valeurs
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
    // FIN SOLUTION
}

/// **Exercice 2.5 — Traits et objets-traits** (chapitre 7)
pub trait Forme {
    fn aire(&self) -> f64;

    /// Nom de la forme. Implémentation par défaut : `"forme"`.
    fn nom(&self) -> String {
        // SOLUTION
        String::from("forme")
        // FIN SOLUTION
    }
}

pub struct Cercle {
    pub rayon: f64,
}

pub struct Rect {
    pub largeur: f64,
    pub hauteur: f64,
}

impl Forme for Cercle {
    fn aire(&self) -> f64 {
        // SOLUTION
        std::f64::consts::PI * self.rayon * self.rayon
        // FIN SOLUTION
    }

    /// Un cercle doit se présenter comme `"cercle"` (redéfinit la méthode par défaut).
    fn nom(&self) -> String {
        // SOLUTION
        String::from("cercle")
        // FIN SOLUTION
    }
}

impl Forme for Rect {
    fn aire(&self) -> f64 {
        // SOLUTION
        self.largeur * self.hauteur
        // FIN SOLUTION
    }
}

/// Somme des aires d'une collection hétérogène de formes.
pub fn aire_totale(formes: &[Box<dyn Forme>]) -> f64 {
    // SOLUTION
    formes.iter().map(|f| f.aire()).sum()
    // FIN SOLUTION
}

/// **Exercice 2.6 — `Box` et types récursifs** (chapitre 10)
///
/// Liste chaînée immuable : `Liste::nouvelle().empiler(1).empiler(2)` représente `2 → 1`.
#[derive(Debug, PartialEq)]
pub enum Liste {
    Vide,
    Cons(i32, Box<Liste>),
}

impl Liste {
    pub fn nouvelle() -> Liste {
        // SOLUTION
        Liste::Vide
        // FIN SOLUTION
    }

    /// Ajoute `valeur` EN TÊTE et retourne la nouvelle liste.
    pub fn empiler(self, valeur: i32) -> Liste {
        // SOLUTION
        Liste::Cons(valeur, Box::new(self))
        // FIN SOLUTION
    }

    /// Somme des éléments (récursif).
    pub fn somme(&self) -> i32 {
        // SOLUTION
        match self {
            Liste::Vide => 0,
            Liste::Cons(valeur, reste) => valeur + reste.somme(),
        }
        // FIN SOLUTION
    }

    /// Éléments de la tête vers la queue.
    pub fn vers_vec(&self) -> Vec<i32> {
        // SOLUTION
        let mut resultat = Vec::new();
        let mut courant = self;
        while let Liste::Cons(valeur, reste) = courant {
            resultat.push(*valeur);
            courant = reste;
        }
        resultat
        // FIN SOLUTION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifetimes() {
        assert_eq!(plus_long("abc", "de"), "abc");
        assert_eq!(plus_long("ab", "cde"), "cde");
        assert_eq!(plus_long("ab", "cd"), "ab");
    }

    #[test]
    fn pile() {
        let mut p: Pile<i32> = Pile::new();
        assert!(p.est_vide());
        assert_eq!(p.depiler(), None);
        p.empiler(1);
        p.empiler(2);
        assert_eq!(p.taille(), 2);
        assert_eq!(p.sommet(), Some(&2));
        assert_eq!(p.depiler(), Some(2));
        assert_eq!(p.depiler(), Some(1));
        assert!(p.est_vide());
    }

    #[test]
    fn pile_de_chaines() {
        let mut p = Pile::new();
        p.empiler(String::from("a"));
        assert_eq!(p.sommet().map(String::as_str), Some("a"));
    }

    #[test]
    fn composition() {
        let plus_un_puis_double = composer(|x: i32| x + 1, |x: i32| x * 2);
        assert_eq!(plus_un_puis_double(5), 12);
        let longueur_puis_texte = composer(|s: &str| s.len(), |n: usize| format!("{n} caractères"));
        assert_eq!(longueur_puis_texte("rust"), "4 caractères");
    }

    #[test]
    fn iterateurs() {
        assert_eq!(somme_carres_pairs(&[1, 2, 3, 4, 5, 6]), 4 + 16 + 36);
        assert_eq!(somme_carres_pairs(&[]), 0);
        assert_eq!(somme_carres_pairs(&[1, 3, 5]), 0);
        assert_eq!(somme_carres_pairs(&[-2, 2]), 8);
    }

    #[test]
    fn formes() {
        let c = Cercle { rayon: 1.0 };
        let r = Rect {
            largeur: 2.0,
            hauteur: 3.0,
        };
        assert!((c.aire() - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(r.aire(), 6.0);
        assert_eq!(c.nom(), "cercle");
        assert_eq!(r.nom(), "forme");

        let toutes: Vec<Box<dyn Forme>> = vec![Box::new(c), Box::new(r)];
        assert!((aire_totale(&toutes) - (std::f64::consts::PI + 6.0)).abs() < 1e-9);
        assert_eq!(aire_totale(&[]), 0.0);
    }

    #[test]
    fn liste_chainee() {
        let l = Liste::nouvelle().empiler(1).empiler(2).empiler(3);
        assert_eq!(l.somme(), 6);
        assert_eq!(l.vers_vec(), vec![3, 2, 1]);
        assert_eq!(Liste::nouvelle().somme(), 0);
        assert_eq!(Liste::nouvelle().vers_vec(), Vec::<i32>::new());
        assert_eq!(
            Liste::nouvelle().empiler(7),
            Liste::Cons(7, Box::new(Liste::Vide))
        );
    }
}
