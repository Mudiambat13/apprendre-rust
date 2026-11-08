//! # Le jeu de la vie de Conway
//!
//! Automate cellulaire sur une grille **torique** (les bords se rejoignent). À chaque génération :
//!
//! - une cellule vivante avec 2 ou 3 voisines vivantes **survit**, sinon elle meurt ;
//! - une cellule morte avec exactement 3 voisines vivantes **naît**.
//!
//! Quatre règles, et pourtant : oscillateurs, vaisseaux (les « planeurs »), canons…
//! Concepts Rust : grille plate dans un `Vec<bool>`, arithmétique modulaire, `Display`.

use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grille {
    largeur: usize,
    hauteur: usize,
    cellules: Vec<bool>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErreurMotif {
    LigneTropLongue { ligne: usize },
    CaractereInconnu(char),
}

impl fmt::Display for ErreurMotif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurMotif::LigneTropLongue { ligne } => {
                write!(f, "la ligne {ligne} dépasse la largeur de la grille")
            }
            ErreurMotif::CaractereInconnu(c) => write!(
                f,
                "caractère inconnu dans le motif : {c:?} (utilisez '#' et '.')"
            ),
        }
    }
}

impl std::error::Error for ErreurMotif {}

impl Grille {
    pub fn new(largeur: usize, hauteur: usize) -> Self {
        assert!(largeur > 0 && hauteur > 0);
        Grille {
            largeur,
            hauteur,
            cellules: vec![false; largeur * hauteur],
        }
    }

    pub fn largeur(&self) -> usize {
        self.largeur
    }

    pub fn hauteur(&self) -> usize {
        self.hauteur
    }

    pub fn est_vivante(&self, x: usize, y: usize) -> bool {
        self.cellules[y * self.largeur + x]
    }

    pub fn definir(&mut self, x: usize, y: usize, vivante: bool) {
        self.cellules[y * self.largeur + x] = vivante;
    }

    pub fn population(&self) -> usize {
        self.cellules.iter().filter(|&&c| c).count()
    }

    /// Place un motif (`#` = vivante, `.` = morte, lignes séparées par `\n`) à partir de `(x0, y0)`.
    pub fn placer(&mut self, motif: &str, x0: usize, y0: usize) -> Result<(), ErreurMotif> {
        for (dy, ligne) in motif
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .enumerate()
        {
            if ligne.chars().count() > self.largeur {
                return Err(ErreurMotif::LigneTropLongue { ligne: dy + 1 });
            }
            for (dx, c) in ligne.chars().enumerate() {
                let vivante = match c {
                    '#' | 'O' => true,
                    '.' => false,
                    autre => return Err(ErreurMotif::CaractereInconnu(autre)),
                };
                // `%` : le motif « s'enroule » si on dépasse le bord.
                self.definir((x0 + dx) % self.largeur, (y0 + dy) % self.hauteur, vivante);
            }
        }
        Ok(())
    }

    fn voisines_vivantes(&self, x: usize, y: usize) -> usize {
        let mut n = 0;
        for dy in [self.hauteur - 1, 0, 1] {
            for dx in [self.largeur - 1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                // « + largeur - 1 » puis modulo = « -1 » sans passer par un entier négatif.
                if self.est_vivante((x + dx) % self.largeur, (y + dy) % self.hauteur) {
                    n += 1;
                }
            }
        }
        n
    }

    /// Génération suivante (la grille actuelle n'est pas modifiée).
    pub fn suivante(&self) -> Grille {
        let mut resultat = Grille::new(self.largeur, self.hauteur);
        for y in 0..self.hauteur {
            for x in 0..self.largeur {
                let voisines = self.voisines_vivantes(x, y);
                let vivra = matches!(
                    (self.est_vivante(x, y), voisines),
                    (true, 2 | 3) | (false, 3)
                );
                resultat.definir(x, y, vivra);
            }
        }
        resultat
    }
}

impl fmt::Display for Grille {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.hauteur {
            let ligne: String = (0..self.largeur)
                .map(|x| if self.est_vivante(x, y) { '█' } else { '·' })
                .collect();
            writeln!(f, "{ligne}")?;
        }
        Ok(())
    }
}

/// Quelques motifs célèbres.
pub mod motifs {
    pub const PLANEUR: &str = ".#.\n..#\n###";
    pub const CLIGNOTANT: &str = "###";
    pub const BLOC: &str = "##\n##";
    pub const CRAPAUD: &str = ".###\n###.";
    pub const R_PENTOMINO: &str = ".##\n##.\n.#.";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grille(motif: &str, l: usize, h: usize, x: usize, y: usize) -> Grille {
        let mut g = Grille::new(l, h);
        g.placer(motif, x, y).unwrap();
        g
    }

    #[test]
    fn le_bloc_est_stable() {
        let g = grille(motifs::BLOC, 6, 6, 2, 2);
        assert_eq!(g.suivante(), g);
    }

    #[test]
    fn le_clignotant_a_une_periode_de_2() {
        let g0 = grille(motifs::CLIGNOTANT, 7, 7, 2, 3);
        let g1 = g0.suivante();
        assert_ne!(g1, g0);
        assert_eq!(g1.population(), 3);
        assert_eq!(g1.suivante(), g0);
    }

    #[test]
    fn le_crapaud_a_une_periode_de_2() {
        let g0 = grille(motifs::CRAPAUD, 8, 8, 2, 3);
        assert_eq!(g0.suivante().suivante(), g0);
        assert_ne!(g0.suivante(), g0);
    }

    #[test]
    fn le_planeur_avance_d_une_case_en_diagonale_tous_les_4_pas() {
        let g0 = grille(motifs::PLANEUR, 10, 10, 1, 1);
        let mut g = g0.clone();
        for _ in 0..4 {
            g = g.suivante();
        }
        assert_eq!(g, grille(motifs::PLANEUR, 10, 10, 2, 2));
        assert_eq!(g.population(), 5);
    }

    #[test]
    fn la_grille_est_torique() {
        // Un planeur finit par revenir à sa position après 4 × largeur générations.
        let g0 = grille(motifs::PLANEUR, 8, 8, 1, 1);
        let mut g = g0.clone();
        for _ in 0..(4 * 8) {
            g = g.suivante();
        }
        assert_eq!(g, g0);
    }

    #[test]
    fn la_grille_vide_reste_vide() {
        let g = Grille::new(5, 5);
        assert_eq!(g.suivante().population(), 0);
    }

    #[test]
    fn motifs_invalides() {
        let mut g = Grille::new(3, 3);
        assert_eq!(
            g.placer("####", 0, 0),
            Err(ErreurMotif::LigneTropLongue { ligne: 1 })
        );
        assert_eq!(
            g.placer("#x#", 0, 0),
            Err(ErreurMotif::CaractereInconnu('x'))
        );
    }

    #[test]
    fn affichage() {
        let g = grille("#.", 2, 1, 0, 0);
        assert_eq!(g.to_string(), "█·\n");
    }
}
