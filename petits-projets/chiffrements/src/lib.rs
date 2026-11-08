//! # Chiffrements classiques et cryptanalyse
//!
//! ⚠️ **À but pédagogique uniquement : ces chiffrements sont trivialement cassables.**
//! Pour protéger de vraies données, utilisez des primitives modernes et auditées
//! (AES-GCM, ChaCha20-Poly1305 via `ring`, `aes-gcm`, `chacha20poly1305`…) — ne codez jamais
//! votre propre cryptographie en production.
//!
//! Au programme :
//! - **César** (décalage constant) et **ROT13** ;
//! - **Vigenère** (décalage variable selon une clé) ;
//! - **XOR** avec une clé répétée, et encodage hexadécimal ;
//! - **Cryptanalyse** : casser un César par analyse des fréquences (test du χ²).

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Erreur {
    CleVide,
    CleSansLettre,
    HexInvalide,
}

impl fmt::Display for Erreur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Erreur::CleVide => write!(f, "la clé ne peut pas être vide"),
            Erreur::CleSansLettre => write!(f, "la clé doit contenir au moins une lettre A-Z"),
            Erreur::HexInvalide => write!(f, "chaîne hexadécimale invalide"),
        }
    }
}

impl std::error::Error for Erreur {}

/// Décale une lettre ASCII de `decalage` positions (négatif accepté) ; les autres caractères restent intacts.
fn decaler(c: char, decalage: i32) -> char {
    let base = match c {
        'a'..='z' => b'a',
        'A'..='Z' => b'A',
        _ => return c,
    };
    let rang = i32::from(c as u8 - base);
    // `rem_euclid` : reste toujours positif, même pour un décalage négatif.
    char::from(base + (rang + decalage).rem_euclid(26) as u8)
}

pub fn cesar(texte: &str, decalage: i32) -> String {
    texte.chars().map(|c| decaler(c, decalage)).collect()
}

pub fn dechiffrer_cesar(texte: &str, decalage: i32) -> String {
    cesar(texte, -decalage)
}

/// ROT13 : César de 13 → c'est son propre inverse.
pub fn rot13(texte: &str) -> String {
    cesar(texte, 13)
}

fn decalages_de_cle(cle: &str) -> Result<Vec<i32>, Erreur> {
    if cle.is_empty() {
        return Err(Erreur::CleVide);
    }
    let d: Vec<i32> = cle
        .chars()
        .filter(char::is_ascii_alphabetic)
        .map(|c| i32::from(c.to_ascii_lowercase() as u8 - b'a'))
        .collect();
    if d.is_empty() {
        Err(Erreur::CleSansLettre)
    } else {
        Ok(d)
    }
}

fn vigenere(texte: &str, cle: &str, signe: i32) -> Result<String, Erreur> {
    let d = decalages_de_cle(cle)?;
    let mut i = 0;
    Ok(texte
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let r = decaler(c, signe * d[i % d.len()]);
                i += 1; // la clé n'avance que sur les lettres
                r
            } else {
                c
            }
        })
        .collect())
}

pub fn vigenere_chiffrer(texte: &str, cle: &str) -> Result<String, Erreur> {
    vigenere(texte, cle, 1)
}

pub fn vigenere_dechiffrer(texte: &str, cle: &str) -> Result<String, Erreur> {
    vigenere(texte, cle, -1)
}

/// XOR de chaque octet avec la clé (répétée). Chiffrer et déchiffrer = la même opération.
pub fn xor(donnees: &[u8], cle: &[u8]) -> Result<Vec<u8>, Erreur> {
    if cle.is_empty() {
        return Err(Erreur::CleVide);
    }
    Ok(donnees
        .iter()
        .zip(cle.iter().cycle())
        .map(|(d, k)| d ^ k)
        .collect())
}

pub fn vers_hex(octets: &[u8]) -> String {
    octets.iter().map(|o| format!("{o:02x}")).collect()
}

pub fn depuis_hex(hex: &str) -> Result<Vec<u8>, Erreur> {
    let hex = hex.trim();
    if hex.len() % 2 != 0 || !hex.is_ascii() {
        return Err(Erreur::HexInvalide);
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| Erreur::HexInvalide))
        .collect()
}

/// Fréquences moyennes des lettres en français (en %), de A à Z.
const FREQUENCES_FR: [f64; 26] = [
    7.64, 0.90, 3.26, 3.67, 14.72, 1.07, 0.87, 0.74, 7.53, 0.61, 0.05, 5.46, 2.97, 7.10, 5.80,
    2.52, 1.36, 6.55, 7.95, 7.24, 6.31, 1.83, 0.04, 0.43, 0.19, 0.12,
];

/// Statistique du χ² : plus elle est faible, plus la distribution de lettres ressemble au français.
fn chi2_francais(texte: &str) -> f64 {
    let mut compte = [0usize; 26];
    let mut total = 0usize;
    for c in texte.chars().filter(char::is_ascii_alphabetic) {
        compte[usize::from(c.to_ascii_lowercase() as u8 - b'a')] += 1;
        total += 1;
    }
    if total == 0 {
        return f64::INFINITY;
    }
    (0..26)
        .map(|i| {
            let attendu = FREQUENCES_FR[i] / 100.0 * total as f64;
            let ecart = compte[i] as f64 - attendu;
            ecart * ecart / attendu
        })
        .sum()
}

/// Casse un César **sans connaître la clé** : essaie les 26 décalages et garde le texte le plus « français ».
/// Retourne `(décalage_utilisé_pour_chiffrer, texte_clair)`. Fiable à partir de quelques dizaines de lettres.
pub fn casser_cesar(chiffre: &str) -> (i32, String) {
    (0..26)
        .map(|d| (d, dechiffrer_cesar(chiffre, d)))
        .min_by(|(_, a), (_, b)| chi2_francais(a).total_cmp(&chi2_francais(b)))
        .expect("26 candidats")
}

/// Indice de coïncidence : probabilité que deux lettres tirées au hasard soient identiques.
/// ≈ 0,074 pour du français, ≈ 0,038 pour du texte uniformément aléatoire (Vigenère à clé longue).
pub fn indice_de_coincidence(texte: &str) -> f64 {
    let mut compte = [0usize; 26];
    let mut n = 0usize;
    for c in texte.chars().filter(char::is_ascii_alphabetic) {
        compte[usize::from(c.to_ascii_lowercase() as u8 - b'a')] += 1;
        n += 1;
    }
    if n < 2 {
        return 0.0;
    }
    compte
        .iter()
        .map(|&c| (c * c.saturating_sub(1)) as f64)
        .sum::<f64>()
        / (n * (n - 1)) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXTE: &str = "Le renard brun saute par-dessus le chien paresseux. Dans la forêt, les animaux se préparent pour l'hiver : \
        les écureuils cachent des noisettes, les ours mangent beaucoup avant de dormir, et les oiseaux migrent vers le sud. \
        Cette histoire est très ancienne et elle est racontée aux enfants depuis des générations.";

    #[test]
    fn cesar_classique() {
        assert_eq!(cesar("Veni, vidi, vici !", 3), "Yhql, ylgl, ylfl !");
        assert_eq!(
            dechiffrer_cesar("Yhql, ylgl, ylfl !", 3),
            "Veni, vidi, vici !"
        );
        assert_eq!(cesar("xyz XYZ", 3), "abc ABC");
        assert_eq!(cesar("abc", -1), "zab");
        assert_eq!(cesar("été", 1), "éué"); // les caractères non ASCII ne bougent pas
    }

    #[test]
    fn cesar_aller_retour_pour_tous_les_decalages() {
        for d in -60..60 {
            assert_eq!(dechiffrer_cesar(&cesar(TEXTE, d), d), TEXTE);
        }
        assert_eq!(cesar(TEXTE, 26), TEXTE);
    }

    #[test]
    fn rot13_est_involutif() {
        assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
        assert_eq!(rot13(&rot13(TEXTE)), TEXTE);
    }

    #[test]
    fn vigenere_vecteur_connu() {
        // Exemple classique de Wikipédia.
        assert_eq!(
            vigenere_chiffrer("ATTACKATDAWN", "LEMON").unwrap(),
            "LXFOPVEFRNHR"
        );
        assert_eq!(
            vigenere_dechiffrer("LXFOPVEFRNHR", "LEMON").unwrap(),
            "ATTACKATDAWN"
        );
    }

    #[test]
    fn vigenere_aller_retour_avec_ponctuation() {
        let c = vigenere_chiffrer(TEXTE, "Clé secrète").unwrap();
        assert_ne!(c, TEXTE);
        assert_eq!(vigenere_dechiffrer(&c, "Clé secrète").unwrap(), TEXTE);
    }

    #[test]
    fn cles_invalides() {
        assert_eq!(vigenere_chiffrer("a", ""), Err(Erreur::CleVide));
        assert_eq!(vigenere_chiffrer("a", "123 !"), Err(Erreur::CleSansLettre));
        assert_eq!(xor(b"a", b""), Err(Erreur::CleVide));
    }

    #[test]
    fn xor_est_involutif() {
        let c = xor(b"message secret", b"cle").unwrap();
        assert_ne!(c, b"message secret");
        assert_eq!(xor(&c, b"cle").unwrap(), b"message secret");
        // XOR avec une clé de la longueur du message = masque jetable (one-time pad) :
        // sûr SEULEMENT si la clé est aléatoire, secrète et JAMAIS réutilisée.
        assert_eq!(
            xor(&[0b1010, 0b1100], &[0b0110, 0b0110]).unwrap(),
            vec![0b1100, 0b1010]
        );
    }

    #[test]
    fn hexadecimal() {
        assert_eq!(vers_hex(&[0, 15, 16, 255]), "000f10ff");
        assert_eq!(depuis_hex("000f10ff").unwrap(), vec![0, 15, 16, 255]);
        assert_eq!(depuis_hex("ABCD").unwrap(), vec![0xab, 0xcd]);
        assert_eq!(depuis_hex("abc"), Err(Erreur::HexInvalide));
        assert_eq!(depuis_hex("zz"), Err(Erreur::HexInvalide));
        assert_eq!(depuis_hex("é"), Err(Erreur::HexInvalide));
    }

    #[test]
    fn on_casse_un_cesar_sans_la_cle() {
        for decalage in [1, 7, 13, 19, 25] {
            let (trouve, clair) = casser_cesar(&cesar(TEXTE, decalage));
            assert_eq!(trouve, decalage);
            assert_eq!(clair, TEXTE);
        }
    }

    #[test]
    fn indice_de_coincidence_distingue_les_chiffrements() {
        let ic_clair = indice_de_coincidence(TEXTE);
        let ic_cesar = indice_de_coincidence(&cesar(TEXTE, 5)); // un César préserve la distribution
        let ic_vigenere = indice_de_coincidence(&vigenere_chiffrer(TEXTE, "ZEBULON").unwrap());
        assert!((ic_clair - ic_cesar).abs() < 1e-12);
        assert!(
            ic_vigenere < ic_clair - 0.01,
            "Vigenère aplatit les fréquences : {ic_vigenere} vs {ic_clair}"
        );
    }
}
