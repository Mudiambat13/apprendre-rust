//! # Chapitre 6 — Gestion des erreurs
//!
//! - Erreurs **irrécupérables** (bugs) → `panic!`
//! - Erreurs **récupérables** (entrées invalides, I/O…) → `Result<T, E>`
//!
//! Lancer : `cargo run --bin ch06_erreurs`

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

// ───────────────────── Un type d'erreur « maison » ─────────────────────

#[derive(Debug)]
enum ErreurConfig {
    /// Ligne mal formée (numéro de ligne, contenu).
    LigneInvalide(usize, String),
    /// Valeur numérique illisible : on garde l'erreur d'origine (`source`).
    ValeurIllisible {
        cle: String,
        source: ParseIntError,
    },
    CleManquante(&'static str),
}

impl fmt::Display for ErreurConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurConfig::LigneInvalide(n, l) => write!(f, "ligne {n} invalide : {l:?}"),
            ErreurConfig::ValeurIllisible { cle, .. } => {
                write!(f, "valeur illisible pour « {cle} »")
            }
            ErreurConfig::CleManquante(c) => write!(f, "clé obligatoire absente : {c}"),
        }
    }
}

impl Error for ErreurConfig {
    // Permet de remonter la chaîne de causes.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErreurConfig::ValeurIllisible { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Config {
    port: u16,
    workers: u32,
}

/// Lit des lignes `cle=valeur`. Le `?` propage l'erreur à l'appelant.
fn lire_config(texte: &str) -> Result<Config, ErreurConfig> {
    let mut paires = HashMap::new();
    for (i, ligne) in texte.lines().enumerate() {
        let ligne = ligne.trim();
        if ligne.is_empty() || ligne.starts_with('#') {
            continue;
        }
        let (cle, valeur) = ligne
            .split_once('=')
            .ok_or_else(|| ErreurConfig::LigneInvalide(i + 1, ligne.to_string()))?;
        paires.insert(cle.trim(), valeur.trim());
    }

    let lire = |cle: &'static str| -> Result<&str, ErreurConfig> {
        paires
            .get(cle)
            .copied()
            .ok_or(ErreurConfig::CleManquante(cle))
    };

    let port = lire("port")?
        .parse()
        .map_err(|source| ErreurConfig::ValeurIllisible {
            cle: "port".into(),
            source,
        })?;
    let workers = lire("workers")?
        .parse()
        .map_err(|source| ErreurConfig::ValeurIllisible {
            cle: "workers".into(),
            source,
        })?;
    Ok(Config { port, workers })
}

// ───────────────── `From` : conversion automatique avec `?` ─────────────────

#[derive(Debug)]
#[allow(dead_code)] // les champs ne sont lus que par le `Debug` dérivé
enum ErreurApp {
    Config(ErreurConfig),
    Calcul(String),
}

impl From<ErreurConfig> for ErreurApp {
    fn from(e: ErreurConfig) -> Self {
        ErreurApp::Config(e)
    }
}

fn demarrer(texte: &str) -> Result<u32, ErreurApp> {
    let cfg = lire_config(texte)?; // ErreurConfig → ErreurApp via From
    if cfg.workers == 0 {
        return Err(ErreurApp::Calcul("il faut au moins un worker".into()));
    }
    Ok(u32::from(cfg.port) / cfg.workers)
}

// ───────────── `Box<dyn Error>` : pratique pour les binaires ─────────────

fn somme_chaine(s: &str) -> Result<i32, Box<dyn Error>> {
    let mut total = 0;
    for morceau in s.split('+') {
        total += morceau.trim().parse::<i32>()?; // ParseIntError → Box<dyn Error>
    }
    if total < 0 {
        return Err("somme négative interdite".into()); // &str → Box<dyn Error>
    }
    Ok(total)
}

fn afficher_chaine_de_causes(e: &dyn Error) {
    println!("  erreur : {e}");
    let mut courant = e.source();
    while let Some(cause) = courant {
        println!("  causée par : {cause}");
        courant = cause.source();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Result et ? ---");
    let bonne = "# serveur\nport = 8080\nworkers = 4";
    println!("{:?}", lire_config(bonne)?);

    for mauvaise in ["port = 80", "port = abc\nworkers = 2", "n'importe quoi"] {
        match lire_config(mauvaise) {
            Ok(c) => println!("ok : {c:?}"),
            Err(e) => afficher_chaine_de_causes(&e),
        }
    }

    println!("--- From ---");
    println!("{:?}", demarrer(bonne));
    println!("{:?}", demarrer("port=1\nworkers=0"));
    println!("{:?}", demarrer("port=1"));

    println!("--- Box<dyn Error> ---");
    println!("{:?}", somme_chaine("1 + 2 + 3")?);
    println!("{}", somme_chaine("1 + x").unwrap_err());
    println!("{}", somme_chaine("-5").unwrap_err());

    println!("--- Combinateurs ---");
    let v: Result<i32, String> = Ok(3);
    println!("{:?}", v.as_ref().map(|x| x * 2));
    println!("{:?}", "12".parse::<i32>().ok()); // Result → Option
    println!("{}", "xx".parse::<i32>().unwrap_or(-1));
    println!("{}", "xx".parse::<i32>().unwrap_or(i32::MAX));
    // Collecter un itérateur de Result : s'arrête à la première erreur.
    let tous: Result<Vec<i32>, _> = ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
    let echec: Result<Vec<i32>, _> = ["1", "oups", "3"]
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{tous:?} / {}", echec.is_err());

    // unwrap()/expect() : OK dans les tests, prototypes, ou quand l'échec est un BUG.
    let port: u16 = "8080".parse().expect("constante valide");
    println!("port = {port}");

    // main peut renvoyer un Result : une Err affiche l'erreur et quitte avec le code 1.
    Ok(())
}
