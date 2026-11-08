//! `cargo run -p labyrinthe -- [largeur] [hauteur] [graine] [--solution]`
//!
//! Exemples : `cargo run -p labyrinthe`  ·  `cargo run -p labyrinthe -- 30 12 7 --solution`

use labyrinthe::Labyrinthe;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let solution = args.iter().any(|a| a == "--solution");
    let nombres: Vec<u64> = args.iter().filter_map(|a| a.parse().ok()).collect();

    let largeur = nombres.first().copied().unwrap_or(20).clamp(1, 200) as usize;
    let hauteur = nombres.get(1).copied().unwrap_or(10).clamp(1, 200) as usize;
    let graine = nombres.get(2).copied().unwrap_or_else(|| {
        // Sans graine : l'heure courante (en nanosecondes).
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(1, |d| d.as_nanos() as u64)
    });

    let lab = Labyrinthe::generer(largeur, hauteur, graine);
    let chemin = lab.resoudre();
    print!(
        "{}",
        lab.dessiner(if solution { chemin.as_deref() } else { None })
    );
    println!(
        "{largeur}×{hauteur} cellules · graine {graine} · plus court chemin : {} pas{}",
        chemin.as_ref().map_or(0, |c| c.len() - 1),
        if solution {
            ""
        } else {
            " (ajoutez --solution pour le tracer)"
        }
    );
}
