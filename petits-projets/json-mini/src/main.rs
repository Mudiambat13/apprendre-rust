//! Reformate du JSON : `cargo run -p json-mini -- [--compact] [fichier]` (stdin si aucun fichier).
//!
//! ```text
//! echo '{"a":[1,2,{"b":null}]}' | cargo run -q -p json-mini
//! cargo run -q -p json-mini -- --compact donnees.json
//! cargo run -q -p json-mini -- --chemin auteurs.0.nom donnees.json
//! ```

use std::io::Read;
use std::process::ExitCode;
use std::{env, fs, io};

use json_mini::Json;

fn main() -> ExitCode {
    let mut compact = false;
    let mut chemin: Option<String> = None;
    let mut fichier: Option<String> = None;

    let mut args = env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--compact" => compact = true,
            "--chemin" => chemin = args.next(),
            _ => fichier = Some(a),
        }
    }

    let texte = match &fichier {
        Some(f) => fs::read_to_string(f).map_err(|e| format!("lecture de « {f} » : {e}")),
        None => {
            let mut s = String::new();
            io::stdin()
                .read_to_string(&mut s)
                .map(|_| s)
                .map_err(|e| format!("lecture de l'entrée : {e}"))
        }
    };
    let texte = match texte {
        Ok(t) => t,
        Err(e) => {
            eprintln!("erreur : {e}");
            return ExitCode::FAILURE;
        }
    };

    let json = match Json::analyser(&texte) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("JSON invalide : {e}");
            return ExitCode::FAILURE;
        }
    };

    let selection = match &chemin {
        Some(c) => match json.chemin(c) {
            Some(v) => v,
            None => {
                eprintln!("chemin introuvable : {c}");
                return ExitCode::FAILURE;
            }
        },
        None => &json,
    };
    println!(
        "{}",
        if compact {
            selection.to_string()
        } else {
            selection.joli()
        }
    );
    ExitCode::SUCCESS
}
