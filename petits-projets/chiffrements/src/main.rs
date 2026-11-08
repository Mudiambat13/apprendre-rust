//! Outil en ligne de commande (⚠ pédagogique, pas de sécurité réelle).
//!
//! ```text
//! chiffrements cesar <décalage> <texte>
//! chiffrements rot13 <texte>
//! chiffrements vigenere <chiffrer|dechiffrer> <clé> <texte>
//! chiffrements xor <clé> <texte>          → hexadécimal
//! chiffrements xor-inverse <clé> <hex>    → texte
//! chiffrements casser <texte chiffré en français>
//! ```

use std::env;
use std::process::ExitCode;

use chiffrements::*;

fn utilisation() -> ExitCode {
    eprintln!(
        "usage :\n  chiffrements cesar <décalage> <texte>\n  chiffrements rot13 <texte>\n  \
         chiffrements vigenere <chiffrer|dechiffrer> <clé> <texte>\n  chiffrements xor <clé> <texte>\n  \
         chiffrements xor-inverse <clé> <hex>\n  chiffrements casser <texte>"
    );
    ExitCode::from(2)
}

fn executer(args: &[String]) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    Ok(match a.as_slice() {
        ["cesar", d, texte] => Some(cesar(texte, d.parse()?)),
        ["rot13", texte] => Some(rot13(texte)),
        ["vigenere", "chiffrer", cle, texte] => Some(vigenere_chiffrer(texte, cle)?),
        ["vigenere", "dechiffrer", cle, texte] => Some(vigenere_dechiffrer(texte, cle)?),
        ["xor", cle, texte] => Some(vers_hex(&xor(texte.as_bytes(), cle.as_bytes())?)),
        ["xor-inverse", cle, hex] => {
            let octets = xor(&depuis_hex(hex)?, cle.as_bytes())?;
            Some(String::from_utf8_lossy(&octets).into_owned())
        }
        ["casser", texte] => {
            let (d, clair) = casser_cesar(texte);
            Some(format!("décalage probable : {d}\n{clair}"))
        }
        _ => None,
    })
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match executer(&args) {
        Ok(Some(sortie)) => {
            println!("{sortie}");
            ExitCode::SUCCESS
        }
        Ok(None) => utilisation(),
        Err(e) => {
            eprintln!("erreur : {e}");
            ExitCode::FAILURE
        }
    }
}
