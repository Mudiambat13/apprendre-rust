//! # Labyrinthes : génération (DFS) et résolution (BFS)
//!
//! - **Génération** : *recursive backtracker* (parcours en profondeur itératif) → un labyrinthe
//!   « parfait » : exactement un chemin entre deux cellules quelconques (un arbre couvrant).
//! - **Résolution** : parcours en largeur (BFS) → plus court chemin garanti.
//!
//! La grille fait `(2·largeur+1) × (2·hauteur+1)` : les cellules sont aux coordonnées impaires,
//! les murs entre elles aux coordonnées paires.
//!
//! Concepts Rust : `Vec` en grille plate, `VecDeque`, générateur pseudo-aléatoire maison.

use std::collections::VecDeque;
use std::fmt;

/// Générateur pseudo-aléatoire xorshift64* : minuscule, déterministe, sans dépendance.
/// (Non cryptographique.)
pub struct Alea(u64);

impl Alea {
    pub fn new(graine: u64) -> Self {
        // L'état ne doit jamais être 0.
        Alea(graine.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub fn suivant(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Entier dans `0..n` (biais négligeable pour de petits `n`).
    pub fn jusqu_a(&mut self, n: usize) -> usize {
        (self.suivant() % n as u64) as usize
    }
}

pub type Point = (usize, usize);

pub struct Labyrinthe {
    largeur: usize, // en cellules
    hauteur: usize,
    ouvert: Vec<bool>, // true = passage, false = mur ; grille (2l+1) × (2h+1)
}

impl Labyrinthe {
    fn largeur_grille(&self) -> usize {
        2 * self.largeur + 1
    }

    fn hauteur_grille(&self) -> usize {
        2 * self.hauteur + 1
    }

    fn indice(&self, (x, y): Point) -> usize {
        y * self.largeur_grille() + x
    }

    pub fn est_ouvert(&self, p: Point) -> bool {
        self.ouvert[self.indice(p)]
    }

    /// Génère un labyrinthe de `largeur × hauteur` cellules (au moins 1×1), reproductible via `graine`.
    pub fn generer(largeur: usize, hauteur: usize, graine: u64) -> Labyrinthe {
        assert!(largeur > 0 && hauteur > 0, "dimensions nulles");
        let mut l = Labyrinthe {
            largeur,
            hauteur,
            ouvert: vec![false; (2 * largeur + 1) * (2 * hauteur + 1)],
        };
        let mut alea = Alea::new(graine);

        let depart = (1, 1);
        let i = l.indice(depart);
        l.ouvert[i] = true;
        let mut pile = vec![depart]; // pile explicite : pas de récursion, donc pas de débordement de pile

        while let Some(&(x, y)) = pile.last() {
            // Voisins non visités à distance 2 (en sautant le mur intermédiaire).
            let mut voisins: Vec<Point> = Vec::with_capacity(4);
            if x >= 3 {
                voisins.push((x - 2, y));
            }
            if y >= 3 {
                voisins.push((x, y - 2));
            }
            if x + 2 < l.largeur_grille() - 1 {
                voisins.push((x + 2, y));
            }
            if y + 2 < l.hauteur_grille() - 1 {
                voisins.push((x, y + 2));
            }
            voisins.retain(|&p| !l.est_ouvert(p));

            if voisins.is_empty() {
                pile.pop(); // impasse : on revient en arrière (backtrack)
            } else {
                let (nx, ny) = voisins[alea.jusqu_a(voisins.len())];
                let (mx, my) = ((x + nx) / 2, (y + ny) / 2); // le mur à abattre
                let (im, i_n) = (l.indice((mx, my)), l.indice((nx, ny)));
                l.ouvert[im] = true;
                l.ouvert[i_n] = true;
                pile.push((nx, ny));
            }
        }
        l
    }

    pub fn entree(&self) -> Point {
        (1, 1)
    }

    pub fn sortie(&self) -> Point {
        (self.largeur_grille() - 2, self.hauteur_grille() - 2)
    }

    /// Plus court chemin de l'entrée à la sortie (BFS), ou `None` s'il n'y en a pas.
    pub fn resoudre(&self) -> Option<Vec<Point>> {
        let (depart, arrivee) = (self.entree(), self.sortie());
        let mut precedent: Vec<Option<Point>> = vec![None; self.ouvert.len()];
        let mut vu = vec![false; self.ouvert.len()];
        let mut file = VecDeque::from([depart]);
        vu[self.indice(depart)] = true;

        while let Some(p) = file.pop_front() {
            if p == arrivee {
                // Remonter le chemin depuis l'arrivée.
                let mut chemin = vec![p];
                let mut courant = p;
                while let Some(prec) = precedent[self.indice(courant)] {
                    chemin.push(prec);
                    courant = prec;
                }
                chemin.reverse();
                return Some(chemin);
            }
            let (x, y) = p;
            let candidats = [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ];
            for q in candidats {
                if q.0 < self.largeur_grille()
                    && q.1 < self.hauteur_grille()
                    && self.est_ouvert(q)
                    && !vu[self.indice(q)]
                {
                    vu[self.indice(q)] = true;
                    precedent[self.indice(q)] = Some(p);
                    file.push_back(q);
                }
            }
        }
        None
    }

    /// Dessin ASCII ; si `chemin` est donné, il est tracé avec des points.
    pub fn dessiner(&self, chemin: Option<&[Point]>) -> String {
        let mut sortie = String::new();
        for y in 0..self.hauteur_grille() {
            for x in 0..self.largeur_grille() {
                let p = (x, y);
                let c = if p == self.entree() {
                    'E'
                } else if p == self.sortie() {
                    'S'
                } else if chemin.is_some_and(|c| c.contains(&p)) {
                    '.'
                } else if self.est_ouvert(p) {
                    ' '
                } else {
                    '#'
                };
                sortie.push(c);
            }
            sortie.push('\n');
        }
        sortie
    }

    pub fn cases_ouvertes(&self) -> usize {
        self.ouvert.iter().filter(|&&o| o).count()
    }
}

impl fmt::Display for Labyrinthe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dessiner(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_generation_est_reproductible() {
        let a = Labyrinthe::generer(10, 8, 42).to_string();
        let b = Labyrinthe::generer(10, 8, 42).to_string();
        let c = Labyrinthe::generer(10, 8, 43).to_string();
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn labyrinthe_parfait_arbre_couvrant() {
        // n cellules reliées par n-1 passages = (2n - 1) cases ouvertes.
        for (l, h, graine) in [(1, 1, 1), (5, 5, 7), (20, 10, 99), (3, 30, 5)] {
            let lab = Labyrinthe::generer(l, h, graine);
            let n = l * h;
            assert_eq!(lab.cases_ouvertes(), 2 * n - 1, "{l}×{h}");
        }
    }

    #[test]
    fn il_existe_toujours_une_solution() {
        for graine in 0..30 {
            let lab = Labyrinthe::generer(15, 15, graine);
            let chemin = lab.resoudre().expect("chemin");
            assert_eq!(chemin.first(), Some(&lab.entree()));
            assert_eq!(chemin.last(), Some(&lab.sortie()));
            // Chaque pas est un déplacement d'une case, sur une case ouverte.
            for paire in chemin.windows(2) {
                let (a, b) = (paire[0], paire[1]);
                assert_eq!(a.0.abs_diff(b.0) + a.1.abs_diff(b.1), 1);
                assert!(lab.est_ouvert(b));
            }
        }
    }

    #[test]
    fn labyrinthe_minimal() {
        let lab = Labyrinthe::generer(1, 1, 0);
        assert_eq!(lab.entree(), lab.sortie());
        assert_eq!(lab.resoudre(), Some(vec![(1, 1)]));
    }

    #[test]
    fn le_bord_reste_ferme() {
        let lab = Labyrinthe::generer(12, 7, 3);
        for x in 0..lab.largeur_grille() {
            assert!(!lab.est_ouvert((x, 0)) && !lab.est_ouvert((x, lab.hauteur_grille() - 1)));
        }
        for y in 0..lab.hauteur_grille() {
            assert!(!lab.est_ouvert((0, y)) && !lab.est_ouvert((lab.largeur_grille() - 1, y)));
        }
    }

    #[test]
    fn dessin_contient_entree_sortie_et_solution() {
        let lab = Labyrinthe::generer(6, 4, 11);
        let chemin = lab.resoudre().unwrap();
        let dessin = lab.dessiner(Some(&chemin));
        assert!(dessin.contains('E') && dessin.contains('S'));
        assert_eq!(dessin.matches('.').count(), chemin.len() - 2);
    }
}
