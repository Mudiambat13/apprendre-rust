//! Animation dans le terminal.
//!
//! `cargo run -p jeu-de-la-vie -- [planeur|clignotant|crapaud|r-pentomino] [générations] [ms]`
//!
//! Exemple : `cargo run -p jeu-de-la-vie -- r-pentomino 200 60`

use std::thread::sleep;
use std::time::Duration;

use jeu_de_la_vie::{Grille, motifs};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let nom = args.first().map_or("planeur", String::as_str);
    let generations: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(60);
    let pause_ms: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(80);

    let (motif, l, h) = match nom {
        "planeur" => (motifs::PLANEUR, 40, 16),
        "clignotant" => (motifs::CLIGNOTANT, 20, 9),
        "crapaud" => (motifs::CRAPAUD, 20, 10),
        "r-pentomino" => (motifs::R_PENTOMINO, 60, 24),
        autre => {
            eprintln!("motif inconnu : {autre} (planeur, clignotant, crapaud, r-pentomino)");
            std::process::exit(2);
        }
    };

    let mut grille = Grille::new(l, h);
    grille
        .placer(motif, l / 2 - 1, h / 2 - 1)
        .expect("le motif tient dans la grille");

    for generation in 0..=generations {
        // Séquence ANSI : efface l'écran et replace le curseur en haut à gauche.
        print!("\x1b[2J\x1b[H");
        println!(
            "{grille}Génération {generation} · population {}   (Ctrl-C pour arrêter)",
            grille.population()
        );
        sleep(Duration::from_millis(pause_ms));
        grille = grille.suivante();
    }
}
