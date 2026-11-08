//! # Compression de Huffman
//!
//! Idée : les octets fréquents reçoivent des codes **courts**, les rares des codes **longs**.
//! L'arbre de Huffman est construit en fusionnant à répétition les deux nœuds de plus faible poids.
//!
//! ## Format du fichier compressé
//!
//! ```text
//! [4 octets] taille d'origine (u32, little-endian)
//! [2 octets] nombre de symboles distincts N (u16)
//! N × [1 octet symbole + 4 octets effectif (u32)]   ← la table de fréquences
//! [reste]    les bits des codes, remplis MSB d'abord, complétés par des 0
//! ```
//!
//! Le décodeur **reconstruit le même arbre** à partir de la table de fréquences : la construction
//! est déterministe (départage des égalités par identifiant de nœud), donc pas besoin de stocker l'arbre.
//!
//! Concepts Rust : `BinaryHeap` + `Reverse`, arène de nœuds (`Vec` + indices au lieu de `Box`),
//! manipulation de bits, `Result` avec erreurs typées.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Erreur {
    EnTeteTronque,
    TableInvalide,
    FluxTronque,
    TropGrand,
}

impl fmt::Display for Erreur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Erreur::EnTeteTronque => write!(f, "en-tête incomplet"),
            Erreur::TableInvalide => write!(f, "table de fréquences invalide"),
            Erreur::FluxTronque => write!(
                f,
                "flux de bits trop court : données tronquées ou corrompues"
            ),
            Erreur::TropGrand => write!(f, "données trop volumineuses (limite : 4 Gio)"),
        }
    }
}

impl std::error::Error for Erreur {}

/// Nœud de l'arbre, stocké dans une arène (`Vec<Noeud>`) et désigné par son indice.
#[derive(Debug, Clone, Copy)]
enum Noeud {
    Feuille(u8),
    Interne { gauche: usize, droite: usize },
}

struct Arbre {
    noeuds: Vec<Noeud>,
    racine: usize,
}

/// Construit l'arbre à partir de `(symbole, effectif)` triés par symbole. `None` si la table est vide.
fn construire_arbre(table: &[(u8, u32)]) -> Option<Arbre> {
    let mut noeuds: Vec<Noeud> = Vec::new();
    // Tas min : (poids, identifiant). L'identifiant départage les égalités → déterminisme.
    let mut tas: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();

    for &(symbole, effectif) in table {
        noeuds.push(Noeud::Feuille(symbole));
        tas.push(Reverse((u64::from(effectif), noeuds.len() - 1)));
    }

    while tas.len() > 1 {
        let Reverse((p1, a)) = tas.pop()?;
        let Reverse((p2, b)) = tas.pop()?;
        noeuds.push(Noeud::Interne {
            gauche: a,
            droite: b,
        });
        tas.push(Reverse((p1 + p2, noeuds.len() - 1)));
    }

    let Reverse((_, racine)) = tas.pop()?;
    Some(Arbre { noeuds, racine })
}

/// Table de codes : pour chaque octet, la suite de bits (`false` = gauche, `true` = droite).
fn table_des_codes(arbre: &Arbre) -> Vec<Option<Vec<bool>>> {
    let mut codes: Vec<Option<Vec<bool>>> = vec![None; 256];
    // Parcours en profondeur avec une pile explicite : (nœud, préfixe).
    let mut pile = vec![(arbre.racine, Vec::<bool>::new())];
    while let Some((i, prefixe)) = pile.pop() {
        match arbre.noeuds[i] {
            Noeud::Feuille(s) => codes[usize::from(s)] = Some(prefixe),
            Noeud::Interne { gauche, droite } => {
                let mut g = prefixe.clone();
                g.push(false);
                let mut d = prefixe;
                d.push(true);
                pile.push((gauche, g));
                pile.push((droite, d));
            }
        }
    }
    codes
}

/// Écrit des bits dans un tampon d'octets, bit de poids fort en premier.
struct EcrivainBits {
    octets: Vec<u8>,
    nb_bits: usize,
}

impl EcrivainBits {
    fn new() -> Self {
        EcrivainBits {
            octets: Vec::new(),
            nb_bits: 0,
        }
    }

    fn ecrire(&mut self, bit: bool) {
        if self.nb_bits % 8 == 0 {
            self.octets.push(0);
        }
        if bit {
            let dernier = self.octets.len() - 1;
            self.octets[dernier] |= 0x80 >> (self.nb_bits % 8);
        }
        self.nb_bits += 1;
    }
}

fn lire_bit(octets: &[u8], position: usize) -> Option<bool> {
    let octet = octets.get(position / 8)?;
    Some(octet & (0x80 >> (position % 8)) != 0)
}

pub fn compresser(donnees: &[u8]) -> Result<Vec<u8>, Erreur> {
    let taille = u32::try_from(donnees.len()).map_err(|_| Erreur::TropGrand)?;

    let mut effectifs = [0u32; 256];
    for &o in donnees {
        effectifs[usize::from(o)] += 1;
    }
    let table: Vec<(u8, u32)> = (0..=255u8)
        .filter(|&s| effectifs[usize::from(s)] > 0)
        .map(|s| (s, effectifs[usize::from(s)]))
        .collect();

    let mut sortie = Vec::new();
    sortie.extend_from_slice(&taille.to_le_bytes());
    sortie.extend_from_slice(&(table.len() as u16).to_le_bytes());
    for &(s, n) in &table {
        sortie.push(s);
        sortie.extend_from_slice(&n.to_le_bytes());
    }

    if let Some(arbre) = construire_arbre(&table) {
        let codes = table_des_codes(&arbre);
        let mut ecrivain = EcrivainBits::new();
        for &o in donnees {
            // Un seul symbole distinct → code vide : tout est dans la table, aucun bit à écrire.
            for &bit in codes[usize::from(o)].as_deref().unwrap_or(&[]) {
                ecrivain.ecrire(bit);
            }
        }
        sortie.extend_from_slice(&ecrivain.octets);
    }
    Ok(sortie)
}

pub fn decompresser(compresse: &[u8]) -> Result<Vec<u8>, Erreur> {
    let entete = compresse.get(..6).ok_or(Erreur::EnTeteTronque)?;
    let taille = u32::from_le_bytes([entete[0], entete[1], entete[2], entete[3]]) as usize;
    let nb_symboles = usize::from(u16::from_le_bytes([entete[4], entete[5]]));
    if nb_symboles > 256 {
        return Err(Erreur::TableInvalide);
    }

    let fin_table = 6 + nb_symboles * 5;
    let brut = compresse.get(6..fin_table).ok_or(Erreur::EnTeteTronque)?;
    let table: Vec<(u8, u32)> = brut
        .chunks_exact(5)
        .map(|c| (c[0], u32::from_le_bytes([c[1], c[2], c[3], c[4]])))
        .collect();

    // La table doit être strictement croissante en symboles, sans effectif nul, et sa somme = taille.
    let valide = table.windows(2).all(|w| w[0].0 < w[1].0)
        && table.iter().all(|&(_, n)| n > 0)
        && table.iter().map(|&(_, n)| u64::from(n)).sum::<u64>() == taille as u64;
    if !valide {
        return Err(Erreur::TableInvalide);
    }

    let Some(arbre) = construire_arbre(&table) else {
        return Ok(Vec::new()); // entrée vide
    };

    // Cas particulier : un seul symbole, répété `taille` fois.
    if let Noeud::Feuille(s) = arbre.noeuds[arbre.racine] {
        return Ok(vec![s; taille]);
    }

    let bits = &compresse[fin_table..];
    // Ne pas pré-allouer `taille` aveuglément : un en-tête malveillant pourrait annoncer 4 Gio.
    let mut sortie = Vec::with_capacity(taille.min(bits.len().saturating_mul(8)));
    let mut position = 0;
    while sortie.len() < taille {
        let mut courant = arbre.racine;
        while let Noeud::Interne { gauche, droite } = arbre.noeuds[courant] {
            let bit = lire_bit(bits, position).ok_or(Erreur::FluxTronque)?;
            position += 1;
            courant = if bit { droite } else { gauche };
        }
        if let Noeud::Feuille(s) = arbre.noeuds[courant] {
            sortie.push(s);
        }
    }
    Ok(sortie)
}

/// Statistiques de compression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stats {
    pub taille_origine: usize,
    pub taille_compressee: usize,
}

impl Stats {
    /// Taux de compression : `compressé / origine` (plus c'est petit, mieux c'est).
    pub fn taux(&self) -> f64 {
        if self.taille_origine == 0 {
            1.0
        } else {
            self.taille_compressee as f64 / self.taille_origine as f64
        }
    }
}

/// Entropie de Shannon (en bits par octet) : borne théorique de la compression symbole par symbole.
pub fn entropie(donnees: &[u8]) -> f64 {
    if donnees.is_empty() {
        return 0.0;
    }
    let mut effectifs = [0usize; 256];
    for &o in donnees {
        effectifs[usize::from(o)] += 1;
    }
    let n = donnees.len() as f64;
    effectifs
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / n;
            -p * p.log2()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aller_retour(donnees: &[u8]) {
        let c = compresser(donnees).unwrap();
        assert_eq!(
            decompresser(&c).unwrap(),
            donnees,
            "entrée de {} octets",
            donnees.len()
        );
    }

    #[test]
    fn cas_limites() {
        aller_retour(b"");
        aller_retour(b"a");
        aller_retour(b"aaaaaaaaaaaaaaaaaaaaaaaa");
        aller_retour(b"ab");
        aller_retour(b"abracadabra");
    }

    #[test]
    fn tous_les_octets() {
        let tous: Vec<u8> = (0..=255).collect();
        aller_retour(&tous);
        let repetes: Vec<u8> = (0..5000).map(|i| (i % 256) as u8).collect();
        aller_retour(&repetes);
    }

    #[test]
    fn texte_francais_se_compresse() {
        let texte =
            "Le langage Rust garantit la sécurité mémoire sans ramasse-miettes. ".repeat(40);
        let c = compresser(texte.as_bytes()).unwrap();
        // ≈ 4 bits/octet d'entropie pour du français + table de fréquences : on attend moins de 60 %.
        assert!(
            c.len() * 10 < texte.len() * 6,
            "{} → {}",
            texte.len(),
            c.len()
        );
        assert_eq!(decompresser(&c).unwrap(), texte.as_bytes());
    }

    #[test]
    fn pseudo_aleatoire() {
        let mut x = 12345u32;
        let donnees: Vec<u8> = (0..4000)
            .map(|_| {
                x = x.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                (x >> 16) as u8
            })
            .collect();
        aller_retour(&donnees);
    }

    #[test]
    fn la_compression_est_deterministe() {
        let d = b"mississippi river";
        assert_eq!(compresser(d).unwrap(), compresser(d).unwrap());
    }

    #[test]
    fn donnees_tronquees() {
        let c = compresser(b"un texte un peu long pour avoir plusieurs octets de bits").unwrap();
        assert_eq!(decompresser(&c[..3]), Err(Erreur::EnTeteTronque));
        assert_eq!(decompresser(&c[..c.len() - 2]), Err(Erreur::FluxTronque));
    }

    #[test]
    fn table_corrompue() {
        let mut c = compresser(b"aabbbc").unwrap();
        c[0] = 99; // la taille annoncée ne correspond plus à la somme des effectifs
        assert_eq!(decompresser(&c), Err(Erreur::TableInvalide));
        assert_eq!(
            decompresser(&[0, 0, 0, 0, 0xFF, 0xFF]),
            Err(Erreur::TableInvalide)
        );
    }

    #[test]
    fn en_tete_malveillant_ne_fait_pas_allouer_des_gigaoctets() {
        // Taille annoncée : 4 Gio, une seule... en fait deux symboles et presque aucun bit.
        let mut faux = vec![0xFF, 0xFF, 0xFF, 0xFF, 2, 0];
        faux.extend_from_slice(&[b'a', 0xFF, 0xFF, 0xFF, 0x7F]);
        faux.extend_from_slice(&[b'b', 0xFF, 0xFF, 0xFF, 0x7F]);
        // Somme des effectifs = 0xFFFFFFFE ≠ taille 0xFFFFFFFF → rejeté sans allocation massive.
        assert_eq!(decompresser(&faux), Err(Erreur::TableInvalide));
    }

    #[test]
    fn l_entropie_borne_la_taille() {
        let d = "abracadabra alakazam ".repeat(30);
        let h = entropie(d.as_bytes());
        assert!(h > 1.0 && h < 4.0);
        // Huffman est optimal parmi les codes par symbole : au plus H + 1 bit par octet (hors en-tête).
        let c = compresser(d.as_bytes()).unwrap();
        let en_tete = 6 + 5 * 8; // 8 symboles distincts ici
        let bits_utiles = (c.len() - en_tete) as f64 * 8.0 / d.len() as f64;
        assert!(bits_utiles < h + 1.0 + 0.1, "{bits_utiles} vs {h}");
    }

    #[test]
    fn stats() {
        let s = Stats {
            taille_origine: 100,
            taille_compressee: 40,
        };
        assert!((s.taux() - 0.4).abs() < 1e-12);
        assert_eq!(
            Stats {
                taille_origine: 0,
                taille_compressee: 6
            }
            .taux(),
            1.0
        );
    }
}
