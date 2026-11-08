//! Une calculatrice en trois étapes classiques, comme un vrai compilateur :
//!
//! ```text
//!  "2 + 3 * x"  ──lexer──►  [2, +, 3, *, x]  ──parser──►  AST  ──évaluateur──►  valeur
//! ```
//!
//! Grammaire (précédence croissante) :
//!
//! ```text
//! instruction := IDENT '=' expr | expr
//! expr        := terme (('+' | '-') terme)*
//! terme       := unaire (('*' | '/' | '%') unaire)*
//! unaire      := '-' unaire | puissance
//! puissance   := primaire ('^' unaire)?           // associative à droite : 2^3^2 = 2^(3^2)
//! primaire    := NOMBRE | IDENT | IDENT '(' args ')' | '(' expr ')'
//! ```

use std::collections::HashMap;
use std::fmt;

// ───────────────────────── Erreurs ─────────────────────────

#[derive(Debug, PartialEq)]
pub enum Erreur {
    CaractereInattendu(char),
    NombreInvalide(String),
    FinInattendue,
    JetonInattendu(String),
    VariableInconnue(String),
    FonctionInconnue(String),
    NombreDArguments {
        fonction: String,
        attendu: usize,
        recu: usize,
    },
    DivisionParZero,
    DomaineInvalide(String),
}

impl fmt::Display for Erreur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Erreur::CaractereInattendu(c) => write!(f, "caractère inattendu : {c:?}"),
            Erreur::NombreInvalide(s) => write!(f, "nombre invalide : {s}"),
            Erreur::FinInattendue => write!(f, "fin d'expression inattendue"),
            Erreur::JetonInattendu(j) => write!(f, "jeton inattendu : {j}"),
            Erreur::VariableInconnue(v) => write!(f, "variable inconnue : {v}"),
            Erreur::FonctionInconnue(n) => write!(f, "fonction inconnue : {n}"),
            Erreur::NombreDArguments {
                fonction,
                attendu,
                recu,
            } => {
                write!(f, "{fonction} attend {attendu} argument(s), {recu} reçu(s)")
            }
            Erreur::DivisionParZero => write!(f, "division par zéro"),
            Erreur::DomaineInvalide(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for Erreur {}

// ───────────────────────── 1. Lexer ─────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Jeton {
    Nombre(f64),
    Ident(String),
    Plus,
    Moins,
    Etoile,
    Barre,
    Pourcent,
    Chapeau,
    ParG,
    ParD,
    Virgule,
    Egal,
}

impl fmt::Display for Jeton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Jeton::Nombre(n) => write!(f, "{n}"),
            Jeton::Ident(s) => write!(f, "{s}"),
            Jeton::Plus => write!(f, "+"),
            Jeton::Moins => write!(f, "-"),
            Jeton::Etoile => write!(f, "*"),
            Jeton::Barre => write!(f, "/"),
            Jeton::Pourcent => write!(f, "%"),
            Jeton::Chapeau => write!(f, "^"),
            Jeton::ParG => write!(f, "("),
            Jeton::ParD => write!(f, ")"),
            Jeton::Virgule => write!(f, ","),
            Jeton::Egal => write!(f, "="),
        }
    }
}

pub fn tokeniser(source: &str) -> Result<Vec<Jeton>, Erreur> {
    let mut jetons = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_whitespace() => {
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut texte = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        texte.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n = texte
                    .parse::<f64>()
                    .map_err(|_| Erreur::NombreInvalide(texte))?;
                jetons.push(Jeton::Nombre(n));
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut nom = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_alphanumeric() || d == '_' {
                        nom.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                jetons.push(Jeton::Ident(nom));
            }
            _ => {
                let jeton = match c {
                    '+' => Jeton::Plus,
                    '-' => Jeton::Moins,
                    '*' => Jeton::Etoile,
                    '/' => Jeton::Barre,
                    '%' => Jeton::Pourcent,
                    '^' => Jeton::Chapeau,
                    '(' => Jeton::ParG,
                    ')' => Jeton::ParD,
                    ',' => Jeton::Virgule,
                    '=' => Jeton::Egal,
                    autre => return Err(Erreur::CaractereInattendu(autre)),
                };
                jetons.push(jeton);
                chars.next();
            }
        }
    }
    Ok(jetons)
}

// ───────────────────────── 2. Parser (descente récursive) ─────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Operateur {
    Ajouter,
    Soustraire,
    Multiplier,
    Diviser,
    Modulo,
    Puissance,
}

/// Arbre de syntaxe abstraite. `Box` est nécessaire pour les types récursifs.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Nombre(f64),
    Variable(String),
    Negation(Box<Expr>),
    Binaire {
        gauche: Box<Expr>,
        op: Operateur,
        droite: Box<Expr>,
    },
    Appel {
        nom: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Affectation(String, Expr),
    Expression(Expr),
}

struct Parser {
    jetons: Vec<Jeton>,
    pos: usize,
}

impl Parser {
    fn courant(&self) -> Option<&Jeton> {
        self.jetons.get(self.pos)
    }

    fn avancer(&mut self) -> Option<Jeton> {
        let j = self.jetons.get(self.pos).cloned();
        self.pos += 1;
        j
    }

    fn attendre(&mut self, attendu: Jeton) -> Result<(), Erreur> {
        match self.avancer() {
            Some(j) if j == attendu => Ok(()),
            Some(j) => Err(Erreur::JetonInattendu(j.to_string())),
            None => Err(Erreur::FinInattendue),
        }
    }

    fn instruction(&mut self) -> Result<Instruction, Erreur> {
        // Affectation : IDENT '=' …
        if let (Some(Jeton::Ident(nom)), Some(Jeton::Egal)) =
            (self.jetons.first(), self.jetons.get(1))
        {
            let nom = nom.clone();
            self.pos = 2;
            let expr = self.expr()?;
            return Ok(Instruction::Affectation(nom, expr));
        }
        Ok(Instruction::Expression(self.expr()?))
    }

    fn expr(&mut self) -> Result<Expr, Erreur> {
        let mut gauche = self.terme()?;
        while let Some(op) = match self.courant() {
            Some(Jeton::Plus) => Some(Operateur::Ajouter),
            Some(Jeton::Moins) => Some(Operateur::Soustraire),
            _ => None,
        } {
            self.avancer();
            let droite = self.terme()?;
            gauche = Expr::Binaire {
                gauche: Box::new(gauche),
                op,
                droite: Box::new(droite),
            };
        }
        Ok(gauche)
    }

    fn terme(&mut self) -> Result<Expr, Erreur> {
        let mut gauche = self.unaire()?;
        while let Some(op) = match self.courant() {
            Some(Jeton::Etoile) => Some(Operateur::Multiplier),
            Some(Jeton::Barre) => Some(Operateur::Diviser),
            Some(Jeton::Pourcent) => Some(Operateur::Modulo),
            _ => None,
        } {
            self.avancer();
            let droite = self.unaire()?;
            gauche = Expr::Binaire {
                gauche: Box::new(gauche),
                op,
                droite: Box::new(droite),
            };
        }
        Ok(gauche)
    }

    fn unaire(&mut self) -> Result<Expr, Erreur> {
        if self.courant() == Some(&Jeton::Moins) {
            self.avancer();
            return Ok(Expr::Negation(Box::new(self.unaire()?)));
        }
        self.puissance()
    }

    fn puissance(&mut self) -> Result<Expr, Erreur> {
        let base = self.primaire()?;
        if self.courant() == Some(&Jeton::Chapeau) {
            self.avancer();
            let exposant = self.unaire()?; // récursion à droite → associativité à droite
            return Ok(Expr::Binaire {
                gauche: Box::new(base),
                op: Operateur::Puissance,
                droite: Box::new(exposant),
            });
        }
        Ok(base)
    }

    fn primaire(&mut self) -> Result<Expr, Erreur> {
        match self.avancer() {
            Some(Jeton::Nombre(n)) => Ok(Expr::Nombre(n)),
            Some(Jeton::Ident(nom)) => {
                if self.courant() == Some(&Jeton::ParG) {
                    self.avancer();
                    let mut args = Vec::new();
                    if self.courant() != Some(&Jeton::ParD) {
                        loop {
                            args.push(self.expr()?);
                            if self.courant() == Some(&Jeton::Virgule) {
                                self.avancer();
                            } else {
                                break;
                            }
                        }
                    }
                    self.attendre(Jeton::ParD)?;
                    Ok(Expr::Appel { nom, args })
                } else {
                    Ok(Expr::Variable(nom))
                }
            }
            Some(Jeton::ParG) => {
                let e = self.expr()?;
                self.attendre(Jeton::ParD)?;
                Ok(e)
            }
            Some(autre) => Err(Erreur::JetonInattendu(autre.to_string())),
            None => Err(Erreur::FinInattendue),
        }
    }
}

pub fn parser(source: &str) -> Result<Instruction, Erreur> {
    let mut p = Parser {
        jetons: tokeniser(source)?,
        pos: 0,
    };
    let instruction = p.instruction()?;
    match p.courant() {
        None => Ok(instruction),
        Some(reste) => Err(Erreur::JetonInattendu(reste.to_string())),
    }
}

// ───────────────────────── 3. Évaluateur ─────────────────────────

/// Environnement : variables définies par l'utilisateur.
pub struct Environnement {
    variables: HashMap<String, f64>,
}

impl Default for Environnement {
    fn default() -> Self {
        let mut variables = HashMap::new();
        variables.insert("pi".to_string(), std::f64::consts::PI);
        variables.insert("e".to_string(), std::f64::consts::E);
        Environnement { variables }
    }
}

impl Environnement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn definir(&mut self, nom: &str, valeur: f64) {
        self.variables.insert(nom.to_string(), valeur);
    }

    pub fn evaluer(&self, expr: &Expr) -> Result<f64, Erreur> {
        match expr {
            Expr::Nombre(n) => Ok(*n),
            Expr::Variable(nom) => self
                .variables
                .get(nom)
                .copied()
                .ok_or_else(|| Erreur::VariableInconnue(nom.clone())),
            Expr::Negation(e) => Ok(-self.evaluer(e)?),
            Expr::Binaire { gauche, op, droite } => {
                let (a, b) = (self.evaluer(gauche)?, self.evaluer(droite)?);
                match op {
                    Operateur::Ajouter => Ok(a + b),
                    Operateur::Soustraire => Ok(a - b),
                    Operateur::Multiplier => Ok(a * b),
                    Operateur::Diviser if b == 0.0 => Err(Erreur::DivisionParZero),
                    Operateur::Diviser => Ok(a / b),
                    Operateur::Modulo if b == 0.0 => Err(Erreur::DivisionParZero),
                    Operateur::Modulo => Ok(a % b),
                    Operateur::Puissance => Ok(a.powf(b)),
                }
            }
            Expr::Appel { nom, args } => {
                let valeurs: Result<Vec<f64>, Erreur> =
                    args.iter().map(|a| self.evaluer(a)).collect();
                appeler(nom, &valeurs?)
            }
        }
    }

    /// Exécute une instruction ; une affectation retourne la valeur affectée.
    pub fn executer(&mut self, instruction: &Instruction) -> Result<f64, Erreur> {
        match instruction {
            Instruction::Expression(e) => self.evaluer(e),
            Instruction::Affectation(nom, e) => {
                let v = self.evaluer(e)?;
                self.variables.insert(nom.clone(), v);
                Ok(v)
            }
        }
    }

    /// Raccourci : texte → valeur.
    pub fn calculer(&mut self, source: &str) -> Result<f64, Erreur> {
        let instruction = parser(source)?;
        self.executer(&instruction)
    }
}

fn appeler(nom: &str, args: &[f64]) -> Result<f64, Erreur> {
    let verifier = |attendu: usize| -> Result<(), Erreur> {
        if args.len() == attendu {
            Ok(())
        } else {
            Err(Erreur::NombreDArguments {
                fonction: nom.to_string(),
                attendu,
                recu: args.len(),
            })
        }
    };
    match nom {
        "sqrt" => {
            verifier(1)?;
            if args[0] < 0.0 {
                return Err(Erreur::DomaineInvalide(
                    "racine carrée d'un nombre négatif".into(),
                ));
            }
            Ok(args[0].sqrt())
        }
        "abs" => verifier(1).map(|_| args[0].abs()),
        "sin" => verifier(1).map(|_| args[0].sin()),
        "cos" => verifier(1).map(|_| args[0].cos()),
        "ln" => {
            verifier(1)?;
            if args[0] <= 0.0 {
                return Err(Erreur::DomaineInvalide(
                    "ln défini seulement pour x > 0".into(),
                ));
            }
            Ok(args[0].ln())
        }
        "max" => verifier(2).map(|_| args[0].max(args[1])),
        "min" => verifier(2).map(|_| args[0].min(args[1])),
        _ => Err(Erreur::FonctionInconnue(nom.to_string())),
    }
}

/// Évaluation « one-shot » sans variables persistantes.
pub fn evaluer(source: &str) -> Result<f64, Erreur> {
    Environnement::new().calculer(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(source: &str) -> f64 {
        evaluer(source).unwrap_or_else(|e| panic!("« {source} » devrait réussir : {e}"))
    }

    fn proche(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn arithmetique_de_base() {
        assert_eq!(ok("1 + 2"), 3.0);
        assert_eq!(ok("10 - 4 - 3"), 3.0); // associativité à gauche
        assert_eq!(ok("2 * 3 + 4"), 10.0); // précédence
        assert_eq!(ok("2 + 3 * 4"), 14.0);
        assert_eq!(ok("7 % 4"), 3.0);
        assert_eq!(ok("1.5 * 2"), 3.0);
    }

    #[test]
    fn parentheses_et_negation() {
        assert_eq!(ok("(2 + 3) * 4"), 20.0);
        assert_eq!(ok("-3 + 5"), 2.0);
        assert_eq!(ok("--4"), 4.0);
        assert_eq!(ok("-(2 + 1)"), -3.0);
    }

    #[test]
    fn puissance_associative_a_droite() {
        assert_eq!(ok("2 ^ 3 ^ 2"), 512.0); // 2^(3^2)
        assert_eq!(ok("-2 ^ 2"), -4.0); // -(2^2)
        assert!(proche(ok("2 ^ -1"), 0.5));
    }

    #[test]
    fn fonctions_et_constantes() {
        assert_eq!(ok("sqrt(16)"), 4.0);
        assert_eq!(ok("max(3, 9) + min(2, 5)"), 11.0);
        assert!(proche(ok("sin(pi / 2)"), 1.0));
        assert!(proche(ok("ln(e)"), 1.0));
        assert_eq!(ok("abs(-7)"), 7.0);
    }

    #[test]
    fn variables() {
        let mut env = Environnement::new();
        assert_eq!(env.calculer("x = 4"), Ok(4.0));
        assert_eq!(env.calculer("y = x * 2"), Ok(8.0));
        assert_eq!(env.calculer("x + y"), Ok(12.0));
    }

    #[test]
    fn erreurs() {
        assert_eq!(evaluer("1 / 0"), Err(Erreur::DivisionParZero));
        assert_eq!(evaluer("x + 1"), Err(Erreur::VariableInconnue("x".into())));
        assert_eq!(
            evaluer("foo(1)"),
            Err(Erreur::FonctionInconnue("foo".into()))
        );
        assert_eq!(evaluer("2 $ 3"), Err(Erreur::CaractereInattendu('$')));
        assert_eq!(evaluer("(1 + 2"), Err(Erreur::FinInattendue));
        assert_eq!(evaluer("1 2"), Err(Erreur::JetonInattendu("2".into())));
        assert_eq!(evaluer("1 +"), Err(Erreur::FinInattendue));
        assert!(matches!(
            evaluer("sqrt(1, 2)"),
            Err(Erreur::NombreDArguments { .. })
        ));
        assert!(matches!(
            evaluer("sqrt(-1)"),
            Err(Erreur::DomaineInvalide(_))
        ));
        assert!(matches!(evaluer("1..2"), Err(Erreur::NombreInvalide(_))));
    }

    #[test]
    fn forme_de_l_arbre() {
        let attendu = Instruction::Expression(Expr::Binaire {
            gauche: Box::new(Expr::Nombre(1.0)),
            op: Operateur::Ajouter,
            droite: Box::new(Expr::Binaire {
                gauche: Box::new(Expr::Nombre(2.0)),
                op: Operateur::Multiplier,
                droite: Box::new(Expr::Nombre(3.0)),
            }),
        });
        assert_eq!(parser("1 + 2 * 3"), Ok(attendu));
    }
}
