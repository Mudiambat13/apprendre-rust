//! Boucle interactive (REPL) de la calculatrice.
//!
//! ```text
//! cargo run -p calculatrice              # mode interactif
//! cargo run -p calculatrice -- "2 + 3*4" # évaluation directe
//! ```

use std::io::{self, BufRead, Write};

use calculatrice::Environnement;

fn main() {
    let mut env = Environnement::new();

    // Mode « one-shot » : chaque argument est une expression.
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        for source in args {
            match env.calculer(&source) {
                Ok(v) => println!("{v}"),
                Err(e) => {
                    eprintln!("erreur : {e}");
                    std::process::exit(1);
                }
            }
        }
        return;
    }

    println!("Calculatrice Rust — opérateurs + - * / % ^, fonctions sqrt abs sin cos ln max min,");
    println!("constantes pi e, variables (x = 3). Tapez « quitter » pour sortir.");

    let stdin = io::stdin();
    let mut sortie = io::stdout();
    loop {
        print!("> ");
        sortie.flush().expect("flush");

        let mut ligne = String::new();
        match stdin.lock().read_line(&mut ligne) {
            Ok(0) => break, // fin de l'entrée (Ctrl-D)
            Ok(_) => {}
            Err(e) => {
                eprintln!("erreur de lecture : {e}");
                break;
            }
        }

        let ligne = ligne.trim();
        match ligne {
            "" => continue,
            "quitter" | "exit" | "q" => break,
            _ => match env.calculer(ligne) {
                Ok(v) => println!("= {v}"),
                Err(e) => println!("erreur : {e}"),
            },
        }
    }
}
