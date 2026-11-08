//! # Chapitre 17 — Patterns et idiomes Rust
//!
//! Newtype, Builder, RAII, machine à états par enum, `Cow`, `mem::take/replace`, `From`/`Into`.
//!
//! Lancer : `cargo run --bin ch17_patterns_idiomes`

#![allow(dead_code)]

use std::borrow::Cow;
use std::fmt;
use std::mem;

fn main() {
    newtype();
    builder();
    raii();
    machine_a_etats();
    cow();
    mem_take();
    conversions();
}

// ───────────── Newtype : des types distincts pour des valeurs de même représentation ─────────────

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Euros(f64);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Dollars(f64);

impl From<Dollars> for Euros {
    fn from(d: Dollars) -> Self {
        Euros(d.0 * 0.92) // taux fictif
    }
}

fn payer(montant: Euros) {
    println!("  paiement de {:.2} €", montant.0);
}

fn newtype() {
    println!("--- Newtype ---");
    // payer(Dollars(10.0)); // ← ne compile pas : impossible de mélanger les devises par erreur
    payer(Dollars(10.0).into());
    // Autre usage : implémenter un trait externe sur un type externe (règle de l'orphelin).
}

// ───────────── Builder : construire des objets complexes pas à pas ─────────────

#[derive(Debug)]
struct Requete {
    url: String,
    methode: String,
    en_tetes: Vec<(String, String)>,
    timeout_s: u32,
    corps: Option<String>,
}

struct RequeteBuilder {
    url: String,
    methode: String,
    en_tetes: Vec<(String, String)>,
    timeout_s: u32,
    corps: Option<String>,
}

impl RequeteBuilder {
    fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            methode: "GET".into(),
            en_tetes: vec![],
            timeout_s: 30,
            corps: None,
        }
    }
    fn methode(mut self, m: &str) -> Self {
        self.methode = m.to_string();
        self
    }
    fn en_tete(mut self, k: &str, v: &str) -> Self {
        self.en_tetes.push((k.into(), v.into()));
        self
    }
    fn timeout(mut self, s: u32) -> Self {
        self.timeout_s = s;
        self
    }
    fn corps(mut self, c: impl Into<String>) -> Self {
        self.corps = Some(c.into());
        self
    }
    /// `build` valide et peut échouer : c'est un des intérêts du pattern.
    fn build(self) -> Result<Requete, String> {
        if self.methode == "GET" && self.corps.is_some() {
            return Err("une requête GET ne peut pas avoir de corps".into());
        }
        Ok(Requete {
            url: self.url,
            methode: self.methode,
            en_tetes: self.en_tetes,
            timeout_s: self.timeout_s,
            corps: self.corps,
        })
    }
}

fn builder() {
    println!("--- Builder ---");
    let r = RequeteBuilder::new("https://example.com/api")
        .methode("POST")
        .en_tete("Content-Type", "application/json")
        .timeout(5)
        .corps(r#"{"ok": true}"#)
        .build();
    println!("{r:#?}");
    println!("{:?}", RequeteBuilder::new("x").corps("oups").build());
}

// ───────────── RAII : acquisition de ressource = initialisation ─────────────

/// Un garde qui exécute une action de nettoyage quand il sort de portée, même en cas de panic
/// ou de `return` anticipé. C'est ainsi que fonctionnent `MutexGuard`, `File`, `Box`…
struct Garde<F: FnMut()> {
    nettoyage: F,
}

impl<F: FnMut()> Drop for Garde<F> {
    fn drop(&mut self) {
        (self.nettoyage)();
    }
}

fn travail_risque(echouer: bool) -> Result<(), String> {
    let _g = Garde {
        nettoyage: || println!("  [nettoyage garanti]"),
    };
    println!("  travail en cours…");
    if echouer {
        return Err("échec".into()); // le nettoyage s'exécute quand même
    }
    Ok(())
}

fn raii() {
    println!("--- RAII ---");
    let _ = travail_risque(false);
    let _ = travail_risque(true);
}

// ───────────── Machine à états : un enum + `match` ─────────────

#[derive(Debug)]
enum Commande {
    Creee,
    Payee { montant: u32 },
    Expediee { suivi: String },
    Annulee,
}

impl Commande {
    fn payer(self, montant: u32) -> Result<Commande, String> {
        match self {
            Commande::Creee => Ok(Commande::Payee { montant }),
            autre => Err(format!("impossible de payer depuis {autre:?}")),
        }
    }
    fn expedier(self, suivi: &str) -> Result<Commande, String> {
        match self {
            Commande::Payee { .. } => Ok(Commande::Expediee {
                suivi: suivi.into(),
            }),
            autre => Err(format!("impossible d'expédier depuis {autre:?}")),
        }
    }
}

fn machine_a_etats() {
    println!("--- Machine à états ---");
    let c = Commande::Creee;
    println!("{:?}", c.payer(50).and_then(|c| c.expedier("ABC123")));
    println!("{:?}", Commande::Creee.expedier("X"));
}

// ───────────── Cow : cloner seulement si nécessaire ─────────────

fn normaliser(entree: &str) -> Cow<'_, str> {
    if entree.contains(' ') {
        Cow::Owned(entree.replace(' ', "_")) // alloue seulement dans ce cas
    } else {
        Cow::Borrowed(entree) // aucune allocation
    }
}

fn cow() {
    println!("--- Cow ---");
    for s in ["déjà_ok", "avec des espaces"] {
        let r = normaliser(s);
        let genre = match &r {
            Cow::Borrowed(_) => "emprunté",
            Cow::Owned(_) => "possédé",
        };
        println!("  {r} ({genre})");
    }
}

// ───────────── mem::take / replace / swap : déplacer depuis un `&mut` ─────────────

#[derive(Debug, Default)]
struct Tampon {
    donnees: Vec<u8>,
    nom: String,
}

impl Tampon {
    /// On ne peut pas « déplacer » un champ hors de `&mut self`… mais on peut le REMPLACER.
    fn vider(&mut self) -> Vec<u8> {
        mem::take(&mut self.donnees) // laisse `Vec::default()` à la place
    }
    fn renommer(&mut self, nouveau: &str) -> String {
        mem::replace(&mut self.nom, nouveau.to_string())
    }
}

fn mem_take() {
    println!("--- mem::take / replace ---");
    let mut t = Tampon {
        donnees: vec![1, 2, 3],
        nom: "ancien".into(),
    };
    let pris = t.vider();
    let ancien_nom = t.renommer("nouveau");
    println!("{pris:?} {ancien_nom} → {t:?}");

    // Idem pour Option : `take()` laisse None.
    let mut o = Some(5);
    let v = o.take();
    println!("{v:?} {o:?}");
}

// ───────────── From / Into / TryFrom / AsRef : API souples ─────────────

struct Identifiant(String);

impl From<&str> for Identifiant {
    fn from(s: &str) -> Self {
        Identifiant(s.to_lowercase())
    }
}
impl From<u32> for Identifiant {
    fn from(n: u32) -> Self {
        Identifiant(format!("id-{n}"))
    }
}
impl fmt::Display for Identifiant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Accepte tout ce qui est convertible : l'appelant écrit le code le plus naturel.
fn enregistrer(id: impl Into<Identifiant>) {
    println!("  enregistré : {}", id.into());
}

/// `AsRef<str>` : accepte &str, String, Cow, etc., sans allocation.
fn longueur_texte(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

fn conversions() {
    println!("--- From / Into / AsRef ---");
    enregistrer("ALICE");
    enregistrer(42u32);
    println!(
        "  {} {}",
        longueur_texte("abc"),
        longueur_texte(String::from("abcd"))
    );
}
