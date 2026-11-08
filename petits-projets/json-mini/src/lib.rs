//! # Un parseur JSON écrit à la main
//!
//! Descente récursive sur le texte, conforme à la grammaire de [json.org](https://www.json.org) :
//! `null`, booléens, nombres, chaînes (avec échappements et `\uXXXX`, y compris les paires
//! de substitution), tableaux et objets.
//!
//! Points d'attention (à lire dans le code) :
//! - **profondeur maximale** : sans limite, `[[[[…` provoquerait un débordement de pile ;
//! - **erreurs avec position** (ligne, colonne) ;
//! - les objets conservent l'ordre d'insertion (`Vec<(String, Json)>`).
//!
//! Pour du vrai JSON en production : la crate `serde_json`.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Nombre(f64),
    Texte(String),
    Tableau(Vec<Json>),
    Objet(Vec<(String, Json)>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ErreurJson {
    pub message: String,
    pub ligne: usize,
    pub colonne: usize,
}

impl fmt::Display for ErreurJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (ligne {}, colonne {})",
            self.message, self.ligne, self.colonne
        )
    }
}

impl std::error::Error for ErreurJson {}

const PROFONDEUR_MAX: usize = 128;

struct Analyseur<'a> {
    texte: &'a str,
    pos: usize, // position en octets
}

impl<'a> Analyseur<'a> {
    fn erreur<T>(&self, message: impl Into<String>) -> Result<T, ErreurJson> {
        let avant = &self.texte[..self.pos.min(self.texte.len())];
        let ligne = avant.matches('\n').count() + 1;
        let colonne = avant.rsplit('\n').next().map_or(0, |l| l.chars().count()) + 1;
        Err(ErreurJson {
            message: message.into(),
            ligne,
            colonne,
        })
    }

    fn peek(&self) -> Option<char> {
        self.texte[self.pos..].chars().next()
    }

    fn avancer(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn espaces(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\n' | '\r')) {
            self.pos += 1;
        }
    }

    fn attendre(&mut self, attendu: char) -> Result<(), ErreurJson> {
        match self.peek() {
            Some(c) if c == attendu => {
                self.pos += c.len_utf8();
                Ok(())
            }
            Some(c) => self.erreur(format!("« {attendu} » attendu, « {c} » trouvé")),
            None => self.erreur(format!("« {attendu} » attendu, fin du texte")),
        }
    }

    fn mot_cle(&mut self, mot: &str, valeur: Json) -> Result<Json, ErreurJson> {
        if self.texte[self.pos..].starts_with(mot) {
            self.pos += mot.len();
            Ok(valeur)
        } else {
            self.erreur(format!("valeur invalide (« {mot} » attendu ?)"))
        }
    }

    fn valeur(&mut self, profondeur: usize) -> Result<Json, ErreurJson> {
        if profondeur > PROFONDEUR_MAX {
            return self.erreur(format!(
                "imbrication trop profonde (maximum {PROFONDEUR_MAX})"
            ));
        }
        self.espaces();
        match self.peek() {
            None => self.erreur("valeur attendue, fin du texte"),
            Some('n') => self.mot_cle("null", Json::Null),
            Some('t') => self.mot_cle("true", Json::Bool(true)),
            Some('f') => self.mot_cle("false", Json::Bool(false)),
            Some('"') => Ok(Json::Texte(self.chaine()?)),
            Some('[') => self.tableau(profondeur),
            Some('{') => self.objet(profondeur),
            Some('-' | '0'..='9') => self.nombre(),
            Some(c) => self.erreur(format!("caractère inattendu : « {c} »")),
        }
    }

    fn nombre(&mut self) -> Result<Json, ErreurJson> {
        let debut = self.pos;
        let chiffres = |a: &mut Self| {
            let d = a.pos;
            while matches!(a.peek(), Some('0'..='9')) {
                a.pos += 1;
            }
            a.pos - d
        };
        if self.peek() == Some('-') {
            self.pos += 1;
        }
        match self.peek() {
            Some('0') => self.pos += 1, // pas de zéro initial superflu : « 012 » est invalide
            Some('1'..='9') => {
                chiffres(self);
            }
            _ => return self.erreur("chiffre attendu"),
        }
        if self.peek() == Some('.') {
            self.pos += 1;
            if chiffres(self) == 0 {
                return self.erreur("chiffre attendu après la virgule");
            }
        }
        if matches!(self.peek(), Some('e' | 'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some('+' | '-')) {
                self.pos += 1;
            }
            if chiffres(self) == 0 {
                return self.erreur("chiffre attendu dans l'exposant");
            }
        }
        match self.texte[debut..self.pos].parse::<f64>() {
            Ok(n) if n.is_finite() => Ok(Json::Nombre(n)),
            _ => self.erreur("nombre hors limites"),
        }
    }

    fn quatre_hexa(&mut self) -> Result<u32, ErreurJson> {
        let mut v = 0;
        for _ in 0..4 {
            match self.avancer().and_then(|c| c.to_digit(16)) {
                Some(d) => v = v * 16 + d,
                None => {
                    return self.erreur("séquence \\u invalide (4 chiffres hexadécimaux attendus)");
                }
            }
        }
        Ok(v)
    }

    fn chaine(&mut self) -> Result<String, ErreurJson> {
        self.attendre('"')?;
        let mut s = String::new();
        loop {
            match self.avancer() {
                None => return self.erreur("chaîne non terminée"),
                Some('"') => return Ok(s),
                Some('\\') => match self.avancer() {
                    Some('"') => s.push('"'),
                    Some('\\') => s.push('\\'),
                    Some('/') => s.push('/'),
                    Some('b') => s.push('\u{8}'),
                    Some('f') => s.push('\u{c}'),
                    Some('n') => s.push('\n'),
                    Some('r') => s.push('\r'),
                    Some('t') => s.push('\t'),
                    Some('u') => {
                        let premier = self.quatre_hexa()?;
                        let code = if (0xD800..0xDC00).contains(&premier) {
                            // Premier élément d'une paire de substitution : un `\uDC00..` doit suivre.
                            if !self.texte[self.pos..].starts_with("\\u") {
                                return self.erreur("paire de substitution incomplète");
                            }
                            self.pos += 2;
                            let second = self.quatre_hexa()?;
                            if !(0xDC00..0xE000).contains(&second) {
                                return self.erreur("paire de substitution invalide");
                            }
                            0x10000 + ((premier - 0xD800) << 10) + (second - 0xDC00)
                        } else {
                            premier
                        };
                        match char::from_u32(code) {
                            Some(c) => s.push(c),
                            None => return self.erreur("point de code Unicode invalide"),
                        }
                    }
                    _ => return self.erreur("échappement inconnu"),
                },
                Some(c) if (c as u32) < 0x20 => {
                    return self.erreur("caractère de contrôle non échappé dans une chaîne");
                }
                Some(c) => s.push(c),
            }
        }
    }

    fn tableau(&mut self, profondeur: usize) -> Result<Json, ErreurJson> {
        self.attendre('[')?;
        let mut elements = Vec::new();
        self.espaces();
        if self.peek() == Some(']') {
            self.pos += 1;
            return Ok(Json::Tableau(elements));
        }
        loop {
            elements.push(self.valeur(profondeur + 1)?);
            self.espaces();
            match self.avancer() {
                Some(',') => {}
                Some(']') => return Ok(Json::Tableau(elements)),
                _ => {
                    self.pos -= 1.min(self.pos);
                    return self.erreur("« , » ou « ] » attendu");
                }
            }
        }
    }

    fn objet(&mut self, profondeur: usize) -> Result<Json, ErreurJson> {
        self.attendre('{')?;
        let mut membres = Vec::new();
        self.espaces();
        if self.peek() == Some('}') {
            self.pos += 1;
            return Ok(Json::Objet(membres));
        }
        loop {
            self.espaces();
            if self.peek() != Some('"') {
                return self.erreur("nom de membre (chaîne) attendu");
            }
            let cle = self.chaine()?;
            self.espaces();
            self.attendre(':')?;
            let v = self.valeur(profondeur + 1)?;
            membres.push((cle, v));
            self.espaces();
            match self.avancer() {
                Some(',') => {}
                Some('}') => return Ok(Json::Objet(membres)),
                _ => {
                    self.pos -= 1.min(self.pos);
                    return self.erreur("« , » ou « } » attendu");
                }
            }
        }
    }
}

impl Json {
    /// Analyse un document JSON complet (rien d'autre que des espaces ne doit suivre la valeur).
    pub fn analyser(texte: &str) -> Result<Json, ErreurJson> {
        let mut a = Analyseur { texte, pos: 0 };
        let v = a.valeur(0)?;
        a.espaces();
        if a.pos < texte.len() {
            return a.erreur("texte superflu après la valeur");
        }
        Ok(v)
    }

    /// Membre d'un objet.
    pub fn get(&self, cle: &str) -> Option<&Json> {
        match self {
            Json::Objet(m) => m.iter().find(|(k, _)| k == cle).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Élément d'un tableau.
    pub fn index(&self, i: usize) -> Option<&Json> {
        match self {
            Json::Tableau(t) => t.get(i),
            _ => None,
        }
    }

    /// Accès par chemin : `"auteurs.0.nom"`.
    pub fn chemin(&self, chemin: &str) -> Option<&Json> {
        chemin
            .split('.')
            .try_fold(self, |v, segment| match segment.parse::<usize>() {
                Ok(i) if matches!(v, Json::Tableau(_)) => v.index(i),
                _ => v.get(segment),
            })
    }

    pub fn as_str(&self) -> Option<&str> {
        if let Json::Texte(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let Json::Nombre(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Json::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    /// Sérialisation indentée.
    pub fn joli(&self) -> String {
        let mut s = String::new();
        self.ecrire(&mut s, Some(0));
        s
    }

    fn ecrire(&self, s: &mut String, indentation: Option<usize>) {
        let retour = |s: &mut String, niveau: usize| {
            s.push('\n');
            s.push_str(&"  ".repeat(niveau));
        };
        match self {
            Json::Null => s.push_str("null"),
            Json::Bool(b) => s.push_str(if *b { "true" } else { "false" }),
            Json::Nombre(n) => s.push_str(&formater_nombre(*n)),
            Json::Texte(t) => echapper(t, s),
            Json::Tableau(t) if t.is_empty() => s.push_str("[]"),
            Json::Objet(m) if m.is_empty() => s.push_str("{}"),
            Json::Tableau(t) => {
                s.push('[');
                for (i, v) in t.iter().enumerate() {
                    if i > 0 {
                        s.push(',');
                    }
                    if let Some(n) = indentation {
                        retour(s, n + 1);
                    }
                    v.ecrire(s, indentation.map(|n| n + 1));
                }
                if let Some(n) = indentation {
                    retour(s, n);
                }
                s.push(']');
            }
            Json::Objet(m) => {
                s.push('{');
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        s.push(',');
                    }
                    if let Some(n) = indentation {
                        retour(s, n + 1);
                    }
                    echapper(k, s);
                    s.push(':');
                    if indentation.is_some() {
                        s.push(' ');
                    }
                    v.ecrire(s, indentation.map(|n| n + 1));
                }
                if let Some(n) = indentation {
                    retour(s, n);
                }
                s.push('}');
            }
        }
    }
}

/// Sérialisation compacte via `to_string()`.
impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.ecrire(&mut s, None);
        f.write_str(&s)
    }
}

fn formater_nombre(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
    }
}

fn echapper(texte: &str, s: &mut String) {
    s.push('"');
    for c in texte.chars() {
        match c {
            '"' => s.push_str("\\\""),
            '\\' => s.push_str("\\\\"),
            '\n' => s.push_str("\\n"),
            '\r' => s.push_str("\\r"),
            '\t' => s.push_str("\\t"),
            c if (c as u32) < 0x20 => s.push_str(&format!("\\u{:04x}", c as u32)),
            c => s.push(c),
        }
    }
    s.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(t: &str) -> Json {
        Json::analyser(t).unwrap_or_else(|e| panic!("{t:?} devrait être valide : {e}"))
    }

    fn err(t: &str) -> ErreurJson {
        Json::analyser(t).expect_err(&format!("{t:?} devrait être invalide"))
    }

    #[test]
    fn valeurs_simples() {
        assert_eq!(ok("null"), Json::Null);
        assert_eq!(ok(" true "), Json::Bool(true));
        assert_eq!(ok("false"), Json::Bool(false));
        assert_eq!(ok("42"), Json::Nombre(42.0));
        assert_eq!(ok("-3.5e2"), Json::Nombre(-350.0));
        assert_eq!(ok("0.25"), Json::Nombre(0.25));
        assert_eq!(ok("\"salut\""), Json::Texte("salut".into()));
    }

    #[test]
    fn structures() {
        let v = ok(
            r#"{"nom": "Alice", "age": 30, "langages": ["rust", "python"], "actif": true, "adresse": null}"#,
        );
        assert_eq!(v.get("nom").and_then(Json::as_str), Some("Alice"));
        assert_eq!(v.get("age").and_then(Json::as_f64), Some(30.0));
        assert_eq!(
            v.chemin("langages.1").and_then(Json::as_str),
            Some("python")
        );
        assert_eq!(v.get("actif").and_then(Json::as_bool), Some(true));
        assert_eq!(v.get("adresse"), Some(&Json::Null));
        assert_eq!(v.get("absent"), None);
        assert_eq!(ok("[]"), Json::Tableau(vec![]));
        assert_eq!(ok("{}"), Json::Objet(vec![]));
        assert_eq!(
            ok("[1, [2, [3]]]").chemin("1.1.0"),
            Some(&Json::Nombre(3.0))
        );
    }

    #[test]
    fn echappements() {
        assert_eq!(
            ok(r#""a\nb\t\"c\" \\ \/""#),
            Json::Texte("a\nb\t\"c\" \\ /".into())
        );
        assert_eq!(ok(r#""éè""#), Json::Texte("éè".into()));
        // Paire de substitution : U+1F980 (🦀)
        assert_eq!(ok(r#""🦀""#), Json::Texte("🦀".into()));
        assert_eq!(ok("\"héllo 🦀 世界\""), Json::Texte("héllo 🦀 世界".into()));
    }

    #[test]
    fn aller_retour() {
        let sources = [
            r#"{"a":[1,2,{"b":null}],"c":"x\ny","d":-0.5,"e":[],"f":{}}"#,
            "[true,false,null,1e3]",
            r#""🦀 \u0001""#,
        ];
        for s in sources {
            let v = ok(s);
            assert_eq!(ok(&v.to_string()), v, "compact : {s}");
            assert_eq!(ok(&v.joli()), v, "joli : {s}");
        }
    }

    #[test]
    fn serialisation_compacte_et_jolie() {
        let v = ok(r#"{"a": [1, 2], "b": {}}"#);
        assert_eq!(v.to_string(), r#"{"a":[1,2],"b":{}}"#);
        assert_eq!(
            v.joli(),
            "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {}\n}"
        );
    }

    #[test]
    fn nombres_invalides() {
        for t in [
            "01", "+1", "1.", ".5", "1e", "-", "--1", "1.2.3", "0x10", "NaN", "Infinity", "1e999",
        ] {
            err(t);
        }
    }

    #[test]
    fn structures_invalides() {
        for t in [
            "",
            "[1,]",
            "[1 2]",
            "{\"a\" 1}",
            "{\"a\":1,}",
            "{a:1}",
            "[",
            "{\"a\":",
            "nul",
            "truee",
            "[1] x",
            "'a'",
            "\"a",
        ] {
            err(t);
        }
        assert!(
            Json::analyser("\"a\nb\"").is_err(),
            "retour à la ligne non échappé"
        );
        assert!(Json::analyser(r#""\x""#).is_err());
        assert!(Json::analyser(r#""\ud83e""#).is_err(), "substitution seule");
        assert!(Json::analyser(r#""\ud83eA""#).is_err());
        assert!(Json::analyser(r#""\u12""#).is_err());
    }

    #[test]
    fn position_des_erreurs() {
        let e = err("{\n  \"a\": 1,\n  \"b\": ?\n}");
        assert_eq!((e.ligne, e.colonne), (3, 8));
        assert!(e.to_string().contains("ligne 3"));
    }

    #[test]
    fn profondeur_limitee() {
        let profond = "[".repeat(10_000) + &"]".repeat(10_000);
        let e = err(&profond);
        assert!(e.message.contains("trop profonde"));
        // Juste en dessous de la limite : accepté.
        let limite = "[".repeat(100) + &"]".repeat(100);
        ok(&limite);
    }

    #[test]
    fn nombres_formates() {
        assert_eq!(Json::Nombre(3.0).to_string(), "3");
        assert_eq!(Json::Nombre(-0.5).to_string(), "-0.5");
        assert_eq!(Json::Nombre(1e21).to_string(), "1000000000000000000000");
    }
}
