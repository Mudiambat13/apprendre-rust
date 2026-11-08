//! Compresseur de Huffman en ligne de commande.
//!
//! ```text
//! cargo run -p huffman -- c entree.txt sortie.huf    # compresser
//! cargo run -p huffman -- d sortie.huf restaure.txt  # décompresser
//! cargo run -p huffman                               # démonstration
//! ```

use std::process::ExitCode;
use std::{env, fs};

use huffman::{Stats, compresser, decompresser, entropie};

fn demo() {
    let texte =
        "Rust est un langage de programmation qui garantit la sécurité mémoire. ".repeat(50);
    let c = compresser(texte.as_bytes()).expect("compression");
    let s = Stats {
        taille_origine: texte.len(),
        taille_compressee: c.len(),
    };
    println!("Texte d'origine : {} octets", s.taille_origine);
    println!(
        "Compressé       : {} octets ({:.1} % de l'original)",
        s.taille_compressee,
        s.taux() * 100.0
    );
    println!(
        "Entropie        : {:.2} bits/octet (contre 8 sans compression)",
        entropie(texte.as_bytes())
    );
    let restaure = decompresser(&c).expect("décompression");
    println!(
        "Aller-retour    : {}",
        if restaure == texte.as_bytes() {
            "identique ✔"
        } else {
            "DIFFÉRENT ✘"
        }
    );
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        demo();
        return ExitCode::SUCCESS;
    }
    let [mode, entree, sortie] = args.as_slice() else {
        eprintln!("usage : huffman <c|d> <entrée> <sortie>   (ou sans argument pour une démo)");
        return ExitCode::from(2);
    };

    let donnees = match fs::read(entree) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("impossible de lire « {entree} » : {e}");
            return ExitCode::FAILURE;
        }
    };
    let resultat = match mode.as_str() {
        "c" => compresser(&donnees),
        "d" => decompresser(&donnees),
        autre => {
            eprintln!("mode inconnu : {autre} (c ou d)");
            return ExitCode::from(2);
        }
    };
    match resultat.map_err(|e| e.to_string()).and_then(|r| {
        fs::write(sortie, &r)
            .map(|()| r.len())
            .map_err(|e| e.to_string())
    }) {
        Ok(n) => {
            println!("{} octets → {n} octets", donnees.len());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("erreur : {e}");
            ExitCode::FAILURE
        }
    }
}
