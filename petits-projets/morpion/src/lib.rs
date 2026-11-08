//! # Morpion contre une IA imbattable
//!
//! L'IA utilise l'algorithme **minimax** avec élagage alpha-bêta : elle explore tous les
//! futurs possibles de la partie (au plus 9! = 362 880 positions, très peu pour un ordinateur)
//! et choisit le coup qui maximise son résultat en supposant que l'adversaire joue parfaitement.
//!
//! Concepts Rust : `enum`, `Option`, `Copy`, tableaux de taille fixe, récursion, `Display`.

use std::fmt;

/// Les deux joueurs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Joueur {
    X,
    O,
}

impl Joueur {
    pub fn adversaire(self) -> Joueur {
        match self {
            Joueur::X => Joueur::O,
            Joueur::O => Joueur::X,
        }
    }

    fn symbole(self) -> char {
        match self {
            Joueur::X => 'X',
            Joueur::O => 'O',
        }
    }
}

/// Raisons pour lesquelles un coup est refusé.
#[derive(Debug, PartialEq, Eq)]
pub enum ErreurCoup {
    HorsPlateau,
    CaseOccupee,
}

impl fmt::Display for ErreurCoup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurCoup::HorsPlateau => write!(f, "la case doit être comprise entre 1 et 9"),
            ErreurCoup::CaseOccupee => write!(f, "cette case est déjà occupée"),
        }
    }
}

impl std::error::Error for ErreurCoup {}

/// Les 8 lignes gagnantes (indices des cases 0..9, ligne par ligne).
const LIGNES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Plateau {
    cases: [Option<Joueur>; 9],
}

impl Plateau {
    pub fn new() -> Self {
        Self::default()
    }

    /// Joue sur la case `indice` (0 à 8).
    pub fn jouer(&mut self, indice: usize, joueur: Joueur) -> Result<(), ErreurCoup> {
        match self.cases.get_mut(indice) {
            None => Err(ErreurCoup::HorsPlateau),
            Some(Some(_)) => Err(ErreurCoup::CaseOccupee),
            Some(case) => {
                *case = Some(joueur);
                Ok(())
            }
        }
    }

    pub fn gagnant(&self) -> Option<Joueur> {
        LIGNES.iter().find_map(
            |&[a, b, c]| match (self.cases[a], self.cases[b], self.cases[c]) {
                (Some(x), Some(y), Some(z)) if x == y && y == z => Some(x),
                _ => None,
            },
        )
    }

    pub fn est_plein(&self) -> bool {
        self.cases.iter().all(Option::is_some)
    }

    pub fn est_termine(&self) -> bool {
        self.gagnant().is_some() || self.est_plein()
    }

    pub fn coups_legaux(&self) -> Vec<usize> {
        (0..9).filter(|&i| self.cases[i].is_none()).collect()
    }

    /// Meilleur coup pour `joueur` (dont c'est le tour), ou `None` si la partie est finie.
    /// À score égal, on prend la première case : le résultat est déterministe.
    pub fn meilleur_coup(&self, joueur: Joueur) -> Option<usize> {
        if self.est_termine() {
            return None;
        }
        let mut meilleur = None;
        let mut meilleur_score = i32::MIN;
        for coup in self.coups_legaux() {
            let mut suite = self.clone();
            suite.cases[coup] = Some(joueur);
            let score = -suite.negamax(joueur.adversaire(), 1, -10_000, 10_000);
            if score > meilleur_score {
                meilleur_score = score;
                meilleur = Some(coup);
            }
        }
        meilleur
    }

    /// *Negamax* avec élagage alpha-bêta : score du point de vue du joueur dont c'est le tour.
    /// Une victoire rapide vaut plus qu'une victoire tardive (`10 - profondeur`).
    fn negamax(&self, tour: Joueur, profondeur: i32, mut alpha: i32, beta: i32) -> i32 {
        if let Some(g) = self.gagnant() {
            // Le gagnant est forcément l'adversaire de celui qui doit jouer.
            debug_assert_eq!(g, tour.adversaire());
            return -(10 - profondeur);
        }
        if self.est_plein() {
            return 0;
        }
        let mut meilleur = i32::MIN;
        for coup in self.coups_legaux() {
            let mut suite = self.clone();
            suite.cases[coup] = Some(tour);
            let score = -suite.negamax(tour.adversaire(), profondeur + 1, -beta, -alpha);
            meilleur = meilleur.max(score);
            alpha = alpha.max(score);
            if alpha >= beta {
                break; // l'adversaire ne laissera jamais cette branche se produire
            }
        }
        meilleur
    }
}

impl fmt::Display for Plateau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ligne in 0..3 {
            let cellules: Vec<String> = (0..3)
                .map(|col| {
                    let i = ligne * 3 + col;
                    match self.cases[i] {
                        Some(j) => j.symbole().to_string(),
                        None => (i + 1).to_string(), // numéro de la case, pour saisir son coup
                    }
                })
                .collect();
            writeln!(f, " {} ", cellules.join(" | "))?;
            if ligne < 2 {
                writeln!(f, "---+---+---")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plateau_depuis(texte: &str) -> Plateau {
        let mut p = Plateau::new();
        for (i, c) in texte.chars().enumerate() {
            match c {
                'X' => p.cases[i] = Some(Joueur::X),
                'O' => p.cases[i] = Some(Joueur::O),
                _ => {}
            }
        }
        p
    }

    #[test]
    fn detecte_les_victoires() {
        assert_eq!(plateau_depuis("XXXOO....").gagnant(), Some(Joueur::X));
        assert_eq!(plateau_depuis("O..O..O..").gagnant(), Some(Joueur::O));
        assert_eq!(plateau_depuis("X...X...X").gagnant(), Some(Joueur::X));
        assert_eq!(plateau_depuis("XOXOXOOXO").gagnant(), None);
    }

    #[test]
    fn refuse_les_coups_invalides() {
        let mut p = Plateau::new();
        assert_eq!(p.jouer(9, Joueur::X), Err(ErreurCoup::HorsPlateau));
        assert_eq!(p.jouer(4, Joueur::X), Ok(()));
        assert_eq!(p.jouer(4, Joueur::O), Err(ErreurCoup::CaseOccupee));
    }

    #[test]
    fn l_ia_gagne_quand_elle_peut() {
        // O peut gagner en jouant la case 5 (index 5) : O O _ sur la ligne du milieu.
        let p = plateau_depuis("XX.OO.X..");
        assert_eq!(p.meilleur_coup(Joueur::O), Some(5));
    }

    #[test]
    fn l_ia_bloque_l_adversaire() {
        // X menace de gagner en 2 ; O doit bloquer.
        let p = plateau_depuis("XX..O....");
        assert_eq!(p.meilleur_coup(Joueur::O), Some(2));
    }

    #[test]
    fn partie_ia_contre_ia_est_nulle() {
        let mut p = Plateau::new();
        let mut tour = Joueur::X;
        while let Some(coup) = p.meilleur_coup(tour) {
            p.jouer(coup, tour).unwrap();
            tour = tour.adversaire();
        }
        assert_eq!(p.gagnant(), None);
        assert!(p.est_plein());
    }

    /// Preuve par exhaustion : quoi que joue X, l'IA (O) ne perd jamais.
    fn l_ia_ne_perd_jamais(p: &Plateau, tour: Joueur) -> bool {
        if p.gagnant() == Some(Joueur::X) {
            return false;
        }
        if p.est_termine() {
            return true;
        }
        if tour == Joueur::X {
            p.coups_legaux().into_iter().all(|c| {
                let mut s = p.clone();
                s.cases[c] = Some(Joueur::X);
                l_ia_ne_perd_jamais(&s, Joueur::O)
            })
        } else {
            let c = p.meilleur_coup(Joueur::O).unwrap();
            let mut s = p.clone();
            s.cases[c] = Some(Joueur::O);
            l_ia_ne_perd_jamais(&s, Joueur::X)
        }
    }

    #[test]
    fn l_ia_en_second_ne_perd_jamais() {
        assert!(l_ia_ne_perd_jamais(&Plateau::new(), Joueur::X));
    }

    #[test]
    fn affichage() {
        let mut p = Plateau::new();
        p.jouer(0, Joueur::X).unwrap();
        let texte = p.to_string();
        assert!(texte.starts_with(" X | 2 | 3 \n"));
    }
}
