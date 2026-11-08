//! Interface en ligne de commande de `minigrep`.
//!
//! ```text
//! cargo run -p minigrep -- [OPTIONS] MOTIF FICHIER
//!
//!   -i   ignorer la casse (ou IGNORE_CASE=1)
//!   -n   afficher les numéros de ligne
//!   -c   afficher seulement le nombre de lignes
//!   -v   inverser : lignes qui NE contiennent PAS le motif
//! ```

use std::env;
use std::process;

use minigrep::Config;

const AIDE: &str = "\
Usage : minigrep [OPTIONS] MOTIF FICHIER

Options :
  -i   ignorer la casse (ou variable d'environnement IGNORE_CASE=1)
  -n   afficher les numéros de ligne
  -c   afficher seulement le nombre de lignes
  -v   inverser la sélection";

fn main() {
    let ignore_case_env = env::var("IGNORE_CASE").is_ok_and(|v| v != "0");

    let config = match Config::analyser(env::args().skip(1), ignore_case_env) {
        Ok(c) => c,
        Err(e) => {
            // Les erreurs vont sur stderr (eprintln!), pas sur stdout.
            eprintln!("erreur : {e}\n\n{AIDE}");
            process::exit(2);
        }
    };

    match minigrep::executer(&config) {
        Ok(lignes) => lignes.iter().for_each(|l| println!("{l}")),
        Err(e) => {
            eprintln!("erreur : {e}");
            process::exit(1);
        }
    }
}
