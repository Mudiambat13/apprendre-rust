//! Gestionnaire de tâches. Le fichier est `taches.tsv` dans le dossier courant,
//! ou celui indiqué par la variable d'environnement `TODO_FICHIER`.
//!
//! ```text
//! cargo run -q -p todo-cli -- ajouter Écrire le chapitre sur les traits
//! cargo run -q -p todo-cli -- lister
//! cargo run -q -p todo-cli -- fait 1
//! ```

use std::path::PathBuf;
use std::process::ExitCode;
use std::{env, fs, io};

use todo_cli::{Commande, Liste, executer};

const AIDE: &str = "usage : todo <commande>\n\n  ajouter <texte>   ajoute une tâche\n  lister [--tout]   liste les tâches (--tout : y compris terminées)\n  fait <id>         marque comme terminée\n  rouvrir <id>      rouvre une tâche\n  supprimer <id>    supprime une tâche\n  vider-faits       supprime toutes les tâches terminées";

fn main() -> ExitCode {
    let chemin: PathBuf =
        env::var_os("TODO_FICHIER").map_or_else(|| PathBuf::from("taches.tsv"), PathBuf::from);
    let args: Vec<String> = env::args().skip(1).collect();

    let commande = match Commande::analyser(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("erreur : {e}\n\n{AIDE}");
            return ExitCode::from(2);
        }
    };

    // Un fichier absent n'est pas une erreur : c'est simplement une liste vide.
    let contenu = match fs::read_to_string(&chemin) {
        Ok(c) => c,
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => {
            eprintln!("impossible de lire {} : {e}", chemin.display());
            return ExitCode::FAILURE;
        }
    };
    let mut liste = match Liste::depuis_texte(&contenu) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{} : {e}", chemin.display());
            return ExitCode::FAILURE;
        }
    };

    match executer(&mut liste, commande) {
        Ok((message, modifiee)) => {
            if modifiee {
                // Écriture atomique : fichier temporaire puis renommage, pour ne jamais laisser
                // un fichier à moitié écrit en cas de coupure.
                let temporaire = chemin.with_extension("tmp");
                if let Err(e) = fs::write(&temporaire, liste.vers_texte())
                    .and_then(|()| fs::rename(&temporaire, &chemin))
                {
                    eprintln!("impossible d'enregistrer {} : {e}", chemin.display());
                    return ExitCode::FAILURE;
                }
            }
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("erreur : {e}");
            ExitCode::FAILURE
        }
    }
}
