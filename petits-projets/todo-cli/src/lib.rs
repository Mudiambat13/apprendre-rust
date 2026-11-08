//! # Gestionnaire de tâches en ligne de commande
//!
//! ```text
//! todo ajouter Acheter du pain      todo lister           todo lister --tout
//! todo fait 2                       todo rouvrir 2        todo supprimer 3
//! todo vider-faits
//! ```
//!
//! Persistance dans un fichier texte, **une tâche par ligne**, au format `id<TAB>x|-<TAB>texte`
//! (le texte est échappé pour ne jamais contenir de tabulation ni de saut de ligne).
//!
//! Architecture : la bibliothèque est **pure** (aucune E/S) — `Liste` se construit depuis un
//! `&str` et se sérialise en `String` ; seul `main.rs` touche au disque. Résultat : tout est testable.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tache {
    pub id: u32,
    pub fait: bool,
    pub texte: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Liste {
    taches: Vec<Tache>,
    prochain_id: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Erreur {
    /// Ligne de fichier mal formée (numéro de ligne à partir de 1).
    FichierCorrompu {
        ligne: usize,
    },
    TacheInconnue(u32),
    TexteVide,
    CommandeInconnue(String),
    ArgumentManquant(&'static str),
    IdentifiantInvalide(String),
}

impl fmt::Display for Erreur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Erreur::FichierCorrompu { ligne } => {
                write!(f, "fichier de tâches corrompu à la ligne {ligne}")
            }
            Erreur::TacheInconnue(id) => write!(f, "aucune tâche n°{id}"),
            Erreur::TexteVide => write!(f, "le texte de la tâche est vide"),
            Erreur::CommandeInconnue(c) => write!(f, "commande inconnue : « {c} »"),
            Erreur::ArgumentManquant(a) => write!(f, "argument manquant : {a}"),
            Erreur::IdentifiantInvalide(s) => write!(f, "identifiant invalide : « {s} »"),
        }
    }
}

impl std::error::Error for Erreur {}

/// Échappe `\`, tabulation et sauts de ligne pour tenir sur une ligne, séparée par des tabulations.
fn echapper(texte: &str) -> String {
    texte
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn desechapper(texte: &str) -> String {
    let mut s = String::with_capacity(texte.len());
    let mut chars = texte.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            s.push(c);
            continue;
        }
        match chars.next() {
            Some('t') => s.push('\t'),
            Some('n') => s.push('\n'),
            Some('r') => s.push('\r'),
            Some('\\') => s.push('\\'),
            Some(autre) => {
                s.push('\\');
                s.push(autre);
            }
            None => s.push('\\'),
        }
    }
    s
}

impl Liste {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn taches(&self) -> &[Tache] {
        &self.taches
    }

    /// Charge une liste depuis le contenu d'un fichier (vide = liste vide).
    pub fn depuis_texte(contenu: &str) -> Result<Liste, Erreur> {
        let mut liste = Liste::new();
        for (i, ligne) in contenu
            .lines()
            .enumerate()
            .filter(|(_, l)| !l.trim().is_empty())
        {
            let corrompu = || Erreur::FichierCorrompu { ligne: i + 1 };
            let mut champs = ligne.splitn(3, '\t');
            let id: u32 = champs
                .next()
                .and_then(|c| c.parse().ok())
                .ok_or_else(corrompu)?;
            let fait = match champs.next() {
                Some("x") => true,
                Some("-") => false,
                _ => return Err(corrompu()),
            };
            let texte = desechapper(champs.next().ok_or_else(corrompu)?);
            if liste.taches.iter().any(|t| t.id == id) {
                return Err(corrompu()); // identifiant en double
            }
            liste.prochain_id = liste
                .prochain_id
                .max(id.checked_add(1).ok_or_else(corrompu)?);
            liste.taches.push(Tache { id, fait, texte });
        }
        Ok(liste)
    }

    pub fn vers_texte(&self) -> String {
        self.taches
            .iter()
            .map(|t| {
                format!(
                    "{}\t{}\t{}\n",
                    t.id,
                    if t.fait { 'x' } else { '-' },
                    echapper(&t.texte)
                )
            })
            .collect()
    }

    pub fn ajouter(&mut self, texte: &str) -> Result<u32, Erreur> {
        let texte = texte.trim();
        if texte.is_empty() {
            return Err(Erreur::TexteVide);
        }
        let id = self.prochain_id.max(1);
        self.prochain_id = id + 1;
        self.taches.push(Tache {
            id,
            fait: false,
            texte: texte.to_string(),
        });
        Ok(id)
    }

    fn tache_mut(&mut self, id: u32) -> Result<&mut Tache, Erreur> {
        self.taches
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(Erreur::TacheInconnue(id))
    }

    pub fn marquer(&mut self, id: u32, fait: bool) -> Result<(), Erreur> {
        self.tache_mut(id)?.fait = fait;
        Ok(())
    }

    pub fn supprimer(&mut self, id: u32) -> Result<Tache, Erreur> {
        let position = self
            .taches
            .iter()
            .position(|t| t.id == id)
            .ok_or(Erreur::TacheInconnue(id))?;
        Ok(self.taches.remove(position))
    }

    /// Supprime toutes les tâches terminées ; retourne combien.
    pub fn vider_faites(&mut self) -> usize {
        let avant = self.taches.len();
        self.taches.retain(|t| !t.fait);
        avant - self.taches.len()
    }

    pub fn afficher(&self, tout: bool) -> String {
        let lignes: Vec<String> = self
            .taches
            .iter()
            .filter(|t| tout || !t.fait)
            .map(|t| {
                format!(
                    "{:>3}  [{}] {}",
                    t.id,
                    if t.fait { 'x' } else { ' ' },
                    t.texte
                )
            })
            .collect();
        if lignes.is_empty() {
            return if tout {
                "(aucune tâche)".into()
            } else {
                "(rien à faire — bravo !)".into()
            };
        }
        let restantes = self.taches.iter().filter(|t| !t.fait).count();
        format!(
            "{}\n— {restantes} à faire sur {}",
            lignes.join("\n"),
            self.taches.len()
        )
    }
}

/// Une commande de l'utilisateur, déjà analysée.
#[derive(Debug, PartialEq, Eq)]
pub enum Commande {
    Ajouter(String),
    Lister { tout: bool },
    Fait(u32),
    Rouvrir(u32),
    Supprimer(u32),
    ViderFaites,
}

impl Commande {
    pub fn analyser(args: &[String]) -> Result<Commande, Erreur> {
        let (nom, reste) = args
            .split_first()
            .ok_or(Erreur::ArgumentManquant("commande"))?;
        let identifiant = |reste: &[String]| -> Result<u32, Erreur> {
            let s = reste
                .first()
                .ok_or(Erreur::ArgumentManquant("identifiant"))?;
            s.parse()
                .map_err(|_| Erreur::IdentifiantInvalide(s.clone()))
        };
        match nom.as_str() {
            "ajouter" | "add" => Ok(Commande::Ajouter(reste.join(" "))),
            "lister" | "ls" => Ok(Commande::Lister {
                tout: reste.iter().any(|a| a == "--tout"),
            }),
            "fait" | "done" => identifiant(reste).map(Commande::Fait),
            "rouvrir" | "undo" => identifiant(reste).map(Commande::Rouvrir),
            "supprimer" | "rm" => identifiant(reste).map(Commande::Supprimer),
            "vider-faits" => Ok(Commande::ViderFaites),
            autre => Err(Erreur::CommandeInconnue(autre.to_string())),
        }
    }
}

/// Exécute une commande ; retourne le message à afficher et si la liste a été modifiée (donc à sauvegarder).
pub fn executer(liste: &mut Liste, commande: Commande) -> Result<(String, bool), Erreur> {
    Ok(match commande {
        Commande::Ajouter(texte) => {
            let id = liste.ajouter(&texte)?;
            (format!("tâche n°{id} ajoutée"), true)
        }
        Commande::Lister { tout } => (liste.afficher(tout), false),
        Commande::Fait(id) => {
            liste.marquer(id, true)?;
            (format!("tâche n°{id} terminée ✔"), true)
        }
        Commande::Rouvrir(id) => {
            liste.marquer(id, false)?;
            (format!("tâche n°{id} rouverte"), true)
        }
        Commande::Supprimer(id) => {
            let t = liste.supprimer(id)?;
            (format!("tâche n°{id} supprimée : {}", t.texte), true)
        }
        Commande::ViderFaites => {
            let n = liste.vider_faites();
            (format!("{n} tâche(s) terminée(s) supprimée(s)"), n > 0)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    fn lancer(liste: &mut Liste, commande: &str) -> String {
        let c = Commande::analyser(&args(commande)).unwrap();
        executer(liste, c).unwrap().0
    }

    #[test]
    fn ajouter_et_lister() {
        let mut l = Liste::new();
        assert_eq!(
            lancer(&mut l, "ajouter Acheter du pain"),
            "tâche n°1 ajoutée"
        );
        assert_eq!(lancer(&mut l, "ajouter Appeler Marie"), "tâche n°2 ajoutée");
        let sortie = lancer(&mut l, "lister");
        assert!(sortie.contains("1  [ ] Acheter du pain"), "{sortie}");
        assert!(sortie.contains("— 2 à faire sur 2"), "{sortie}");
    }

    #[test]
    fn terminer_rouvrir_filtrer() {
        let mut l = Liste::new();
        lancer(&mut l, "ajouter a");
        lancer(&mut l, "ajouter b");
        lancer(&mut l, "fait 1");
        assert!(!lancer(&mut l, "lister").contains("] a"));
        assert!(lancer(&mut l, "lister --tout").contains("[x] a"));
        lancer(&mut l, "rouvrir 1");
        assert!(lancer(&mut l, "lister").contains("] a"));
    }

    #[test]
    fn les_identifiants_ne_sont_jamais_reutilises() {
        let mut l = Liste::new();
        lancer(&mut l, "ajouter a");
        lancer(&mut l, "ajouter b");
        lancer(&mut l, "supprimer 2");
        assert_eq!(lancer(&mut l, "ajouter c"), "tâche n°3 ajoutée");
    }

    #[test]
    fn vider_les_taches_faites() {
        let mut l = Liste::new();
        for t in ["a", "b", "c"] {
            l.ajouter(t).unwrap();
        }
        l.marquer(1, true).unwrap();
        l.marquer(3, true).unwrap();
        assert_eq!(l.vider_faites(), 2);
        assert_eq!(l.taches().len(), 1);
        assert_eq!(l.vider_faites(), 0);
    }

    #[test]
    fn erreurs_de_commande() {
        assert_eq!(
            Commande::analyser(&[]),
            Err(Erreur::ArgumentManquant("commande"))
        );
        assert_eq!(
            Commande::analyser(&args("danser")),
            Err(Erreur::CommandeInconnue("danser".into()))
        );
        assert_eq!(
            Commande::analyser(&args("fait")),
            Err(Erreur::ArgumentManquant("identifiant"))
        );
        assert_eq!(
            Commande::analyser(&args("fait abc")),
            Err(Erreur::IdentifiantInvalide("abc".into()))
        );
        assert_eq!(
            Commande::analyser(&args("fait -1")),
            Err(Erreur::IdentifiantInvalide("-1".into()))
        );
        let mut l = Liste::new();
        assert_eq!(
            executer(&mut l, Commande::Fait(9)),
            Err(Erreur::TacheInconnue(9))
        );
        assert_eq!(
            executer(&mut l, Commande::Ajouter("   ".into())),
            Err(Erreur::TexteVide)
        );
    }

    #[test]
    fn persistance_aller_retour() {
        let mut l = Liste::new();
        l.ajouter("simple").unwrap();
        l.ajouter("avec\ttabulation et\nsaut de ligne et \\ antislash")
            .unwrap();
        l.ajouter("accents : été 🦀").unwrap();
        l.marquer(2, true).unwrap();
        let texte = l.vers_texte();
        assert_eq!(texte.lines().count(), 3, "une ligne par tâche : {texte:?}");
        assert_eq!(Liste::depuis_texte(&texte).unwrap(), l);
    }

    #[test]
    fn fichier_vide_ou_corrompu() {
        assert_eq!(Liste::depuis_texte("").unwrap(), Liste::new());
        assert_eq!(Liste::depuis_texte("\n\n").unwrap(), Liste::new());
        assert_eq!(
            Liste::depuis_texte("1\t-\tok\nn'importe quoi\n"),
            Err(Erreur::FichierCorrompu { ligne: 2 })
        );
        assert_eq!(
            Liste::depuis_texte("1\tz\ttexte\n"),
            Err(Erreur::FichierCorrompu { ligne: 1 })
        );
        assert_eq!(
            Liste::depuis_texte("1\t-\ta\n1\t-\tb\n"),
            Err(Erreur::FichierCorrompu { ligne: 2 })
        );
        assert_eq!(
            Liste::depuis_texte("4294967295\t-\ta\n"),
            Err(Erreur::FichierCorrompu { ligne: 1 })
        );
    }

    #[test]
    fn le_compteur_reprend_apres_chargement() {
        let mut l = Liste::depuis_texte("7\t-\tancienne\n").unwrap();
        assert_eq!(l.ajouter("nouvelle").unwrap(), 8);
    }

    #[test]
    fn message_de_liste_vide() {
        assert_eq!(Liste::new().afficher(false), "(rien à faire — bravo !)");
        assert_eq!(Liste::new().afficher(true), "(aucune tâche)");
    }

    #[test]
    fn la_modification_est_signalee() {
        let mut l = Liste::new();
        assert!(
            !executer(&mut l, Commande::Lister { tout: false })
                .unwrap()
                .1
        );
        assert!(executer(&mut l, Commande::Ajouter("x".into())).unwrap().1);
        assert!(!executer(&mut l, Commande::ViderFaites).unwrap().1);
    }
}
