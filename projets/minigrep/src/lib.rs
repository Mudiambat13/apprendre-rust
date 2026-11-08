//! Bibliothèque de `minigrep`.
//!
//! Séparer la logique (`lib.rs`) de l'interface (`main.rs`) rend le code **testable** :
//! on ne peut pas facilement tester un `main`, mais on peut tester des fonctions pures.

use std::error::Error;
use std::fmt;
use std::fs;

/// Options de la ligne de commande.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub motif: String,
    pub fichier: String,
    pub ignorer_casse: bool,
    pub numeros_de_ligne: bool,
    pub compter: bool,
    pub inverser: bool,
}

/// Erreurs d'utilisation (mauvais arguments).
#[derive(Debug, PartialEq, Eq)]
pub enum ErreurArguments {
    ArgumentsManquants,
    OptionInconnue(String),
}

impl fmt::Display for ErreurArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurArguments::ArgumentsManquants => write!(f, "arguments manquants"),
            ErreurArguments::OptionInconnue(o) => write!(f, "option inconnue : {o}"),
        }
    }
}

impl Error for ErreurArguments {}

impl Config {
    /// Analyse les arguments (sans le nom du programme).
    ///
    /// `IGNORE_CASE=1` dans l'environnement équivaut à `-i`.
    pub fn analyser<I>(args: I, ignore_case_env: bool) -> Result<Config, ErreurArguments>
    where
        I: IntoIterator<Item = String>,
    {
        let mut positionnels = Vec::new();
        let mut config = Config {
            motif: String::new(),
            fichier: String::new(),
            ignorer_casse: ignore_case_env,
            numeros_de_ligne: false,
            compter: false,
            inverser: false,
        };

        for arg in args {
            match arg.as_str() {
                "-i" => config.ignorer_casse = true,
                "-n" => config.numeros_de_ligne = true,
                "-c" => config.compter = true,
                "-v" => config.inverser = true,
                option if option.starts_with('-') && option.len() > 1 => {
                    return Err(ErreurArguments::OptionInconnue(arg));
                }
                _ => positionnels.push(arg),
            }
        }

        let mut it = positionnels.into_iter();
        config.motif = it.next().ok_or(ErreurArguments::ArgumentsManquants)?;
        config.fichier = it.next().ok_or(ErreurArguments::ArgumentsManquants)?;
        Ok(config)
    }
}

/// Une ligne trouvée, avec son numéro (à partir de 1).
#[derive(Debug, PartialEq, Eq)]
pub struct Correspondance<'a> {
    pub numero: usize,
    pub ligne: &'a str,
}

/// Cherche `motif` (sensible à la casse). Les lignes retournées *empruntent* `contenu`.
pub fn rechercher<'a>(motif: &str, contenu: &'a str) -> Vec<Correspondance<'a>> {
    filtrer(contenu, |ligne| ligne.contains(motif))
}

/// Variante insensible à la casse.
pub fn rechercher_insensible<'a>(motif: &str, contenu: &'a str) -> Vec<Correspondance<'a>> {
    let motif = motif.to_lowercase();
    filtrer(contenu, |ligne| ligne.to_lowercase().contains(&motif))
}

fn filtrer<'a>(contenu: &'a str, predicat: impl Fn(&str) -> bool) -> Vec<Correspondance<'a>> {
    contenu
        .lines()
        .enumerate()
        .filter(|(_, ligne)| predicat(ligne))
        .map(|(i, ligne)| Correspondance {
            numero: i + 1,
            ligne,
        })
        .collect()
}

/// Fonction principale de la logique : lit le fichier et retourne les lignes à afficher.
pub fn executer(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contenu = fs::read_to_string(&config.fichier)
        .map_err(|e| format!("impossible de lire « {} » : {e}", config.fichier))?;
    Ok(formater(config, &contenu))
}

/// Logique pure (sans I/O) : facile à tester.
pub fn formater(config: &Config, contenu: &str) -> Vec<String> {
    let trouvees = if config.ignorer_casse {
        rechercher_insensible(&config.motif, contenu)
    } else {
        rechercher(&config.motif, contenu)
    };

    let retenues: Vec<Correspondance> = if config.inverser {
        let numeros: std::collections::HashSet<usize> = trouvees.iter().map(|c| c.numero).collect();
        contenu
            .lines()
            .enumerate()
            .filter(|(i, _)| !numeros.contains(&(i + 1)))
            .map(|(i, ligne)| Correspondance {
                numero: i + 1,
                ligne,
            })
            .collect()
    } else {
        trouvees
    };

    if config.compter {
        return vec![retenues.len().to_string()];
    }
    retenues
        .into_iter()
        .map(|c| {
            if config.numeros_de_ligne {
                format!("{}:{}", c.numero, c.ligne)
            } else {
                c.ligne.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXTE: &str = "\
Rust est rapide.
Rust est sûr.
Python est pratique.
rust aime la casse.";

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    fn config(motif: &str) -> Config {
        Config {
            motif: motif.into(),
            fichier: "x".into(),
            ignorer_casse: false,
            numeros_de_ligne: false,
            compter: false,
            inverser: false,
        }
    }

    #[test]
    fn sensible_a_la_casse() {
        let r = rechercher("Rust", TEXTE);
        assert_eq!(r.len(), 2);
        assert_eq!(
            r[0],
            Correspondance {
                numero: 1,
                ligne: "Rust est rapide."
            }
        );
    }

    #[test]
    fn insensible_a_la_casse() {
        assert_eq!(rechercher_insensible("RUST", TEXTE).len(), 3);
    }

    #[test]
    fn aucun_resultat() {
        assert!(rechercher("Java", TEXTE).is_empty());
    }

    #[test]
    fn analyse_des_arguments() {
        let c = Config::analyser(args(&["-i", "-n", "motif", "fichier.txt"]), false).unwrap();
        assert_eq!(c.motif, "motif");
        assert_eq!(c.fichier, "fichier.txt");
        assert!(c.ignorer_casse && c.numeros_de_ligne && !c.compter);
    }

    #[test]
    fn variable_d_environnement() {
        let c = Config::analyser(args(&["m", "f"]), true).unwrap();
        assert!(c.ignorer_casse);
    }

    #[test]
    fn erreurs_d_arguments() {
        assert_eq!(
            Config::analyser(args(&["seul"]), false),
            Err(ErreurArguments::ArgumentsManquants)
        );
        assert_eq!(
            Config::analyser(args(&["-z", "a", "b"]), false),
            Err(ErreurArguments::OptionInconnue("-z".into()))
        );
    }

    #[test]
    fn formatage_avec_numeros() {
        let mut c = config("est");
        c.numeros_de_ligne = true;
        let sortie = formater(&c, TEXTE);
        assert_eq!(sortie[0], "1:Rust est rapide.");
        assert_eq!(sortie.len(), 3);
    }

    #[test]
    fn comptage() {
        let mut c = config("Rust");
        c.compter = true;
        assert_eq!(formater(&c, TEXTE), vec!["2"]);
    }

    #[test]
    fn inversion() {
        let mut c = config("Rust");
        c.inverser = true;
        assert_eq!(
            formater(&c, TEXTE),
            vec!["Python est pratique.", "rust aime la casse."]
        );
    }
}
