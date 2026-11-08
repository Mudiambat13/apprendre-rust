//! Jouez au morpion contre l'IA : `cargo run -p morpion`
//! Vous jouez `X` et commencez ; tapez le numéro de la case (1-9), ou `q` pour quitter.
//! Option `--ia-commence` pour laisser l'IA ouvrir.

use std::io::{self, BufRead, Write};

use morpion::{Joueur, Plateau};

fn main() {
    let ia_commence = std::env::args().any(|a| a == "--ia-commence");
    let (humain, ia) = if ia_commence {
        (Joueur::O, Joueur::X)
    } else {
        (Joueur::X, Joueur::O)
    };

    let mut plateau = Plateau::new();
    let mut tour = Joueur::X;
    let stdin = io::stdin();

    println!("Morpion — vous êtes {humain:?}. L'IA est imbattable : visez le match nul !\n");

    while !plateau.est_termine() {
        println!("{plateau}");
        if tour == humain {
            print!("Votre coup (1-9) : ");
            io::stdout().flush().expect("flush");
            let mut ligne = String::new();
            if stdin.lock().read_line(&mut ligne).unwrap_or(0) == 0 {
                return; // fin de l'entrée
            }
            let ligne = ligne.trim();
            if ligne == "q" {
                return;
            }
            match ligne.parse::<usize>() {
                Ok(n) if n >= 1 => match plateau.jouer(n - 1, humain) {
                    Ok(()) => tour = ia,
                    Err(e) => println!("⚠ {e}\n"),
                },
                _ => println!("⚠ tapez un nombre de 1 à 9\n"),
            }
        } else {
            let coup = plateau
                .meilleur_coup(ia)
                .expect("la partie n'est pas finie");
            plateau.jouer(coup, ia).expect("coup de l'IA valide");
            println!("L'IA joue la case {}.\n", coup + 1);
            tour = humain;
        }
    }

    println!("{plateau}");
    match plateau.gagnant() {
        Some(g) if g == humain => println!("🎉 Vous avez gagné (impossible, normalement !)"),
        Some(_) => println!("L'IA gagne."),
        None => println!("Match nul."),
    }
}
