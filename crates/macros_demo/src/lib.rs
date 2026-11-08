//! Trois sortes de macros procédurales, écrites avec la seule crate `proc_macro` :
//!
//! | Sorte | Syntaxe d'appel | Exemple ici |
//! |---|---|---|
//! | dérivée (*derive*) | `#[derive(Describe)]` | [`Describe`] |
//! | attribut | `#[chronometre]` | [`chronometre`] |
//! | fonction | `carre_const!(12)` | [`carre_const`] |
//!
//! ⚠ Code pédagogique : le parsing « à la main » ne gère pas les génériques, les enums, etc.
//! Pour un vrai projet, utilisez `syn` + `quote` (voir `docs/chapitres/20-proc-macros.md`).

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

// ───────────────────────── 1. Macro dérivée ─────────────────────────

/// `#[derive(Describe)]` sur une struct à champs nommés génère :
///
/// ```ignore
/// impl MaStruct {
///     pub fn describe() -> String;                       // "MaStruct { nom: String, age: u32 }"
///     pub fn noms_des_champs() -> &'static [&'static str];
/// }
/// ```
#[proc_macro_derive(Describe)]
pub fn derive_describe(entree: TokenStream) -> TokenStream {
    let mut jetons = entree.into_iter();

    // 1. Chercher le mot-clé `struct`, puis le nom du type.
    let mut nom_type = None;
    for jeton in jetons.by_ref() {
        if let TokenTree::Ident(id) = &jeton {
            if id.to_string() == "struct" {
                if let Some(TokenTree::Ident(nom)) = jetons.next() {
                    nom_type = Some(nom.to_string());
                }
                break;
            }
        }
    }
    let nom_type = nom_type.expect("#[derive(Describe)] ne supporte que les `struct`");

    // 2. Trouver le bloc `{ … }` des champs.
    let mut champs: Vec<(String, String)> = Vec::new();
    for jeton in jetons {
        if let TokenTree::Group(g) = jeton {
            if g.delimiter() == Delimiter::Brace {
                champs = extraire_champs(g.stream());
            }
        }
    }

    // 3. Générer le code (ici sous forme de texte puis re-parsé en `TokenStream`).
    let description = if champs.is_empty() {
        format!("{nom_type} {{}}")
    } else {
        let liste: Vec<String> = champs.iter().map(|(n, t)| format!("{n}: {t}")).collect();
        format!("{nom_type} {{ {} }}", liste.join(", "))
    };
    let noms: Vec<String> = champs.iter().map(|(n, _)| format!("{n:?}")).collect();

    format!(
        "impl {nom_type} {{
            pub fn describe() -> ::std::string::String {{
                ::std::string::String::from({description:?})
            }}
            pub fn noms_des_champs() -> &'static [&'static str] {{
                &[{noms}]
            }}
        }}",
        noms = noms.join(", ")
    )
    .parse()
    .expect("code généré invalide")
}

/// `TokenStream::to_string` insère des espaces entre jetons (`Vec < String >`) : on les retire
/// pour un affichage lisible (`Vec<String>`).
fn normaliser_type(brut: &str) -> String {
    brut.replace(" < ", "<")
        .replace(" <", "<")
        .replace("< ", "<")
        .replace(" >", ">")
        .replace(" ,", ",")
        .replace(" :: ", "::")
        .replace("& ", "&")
}

/// Découpe le contenu d'un `{ … }` en paires (nom, type), en tenant compte des `<…>`
/// (les virgules dans `HashMap<String, u32>` ne séparent pas deux champs).
fn extraire_champs(flux: TokenStream) -> Vec<(String, String)> {
    let mut resultat = Vec::new();
    let mut segment: Vec<TokenTree> = Vec::new();
    let mut profondeur = 0i32;

    let mut terminer = |segment: &mut Vec<TokenTree>| {
        if segment.is_empty() {
            return;
        }
        // Le nom du champ = dernier identifiant avant le premier `:`.
        let mut dernier_ident = None;
        let mut position_deux_points = None;
        for (i, jeton) in segment.iter().enumerate() {
            match jeton {
                TokenTree::Ident(id) => dernier_ident = Some(id.to_string()),
                TokenTree::Punct(p) if p.as_char() == ':' => {
                    position_deux_points = Some(i);
                    break;
                }
                _ => {}
            }
        }
        if let (Some(nom), Some(pos)) = (dernier_ident, position_deux_points) {
            let type_: TokenStream = segment[pos + 1..].iter().cloned().collect();
            resultat.push((nom, normaliser_type(&type_.to_string())));
        }
        segment.clear();
    };

    for jeton in flux {
        match &jeton {
            TokenTree::Punct(p) if p.as_char() == '<' => profondeur += 1,
            TokenTree::Punct(p) if p.as_char() == '>' => profondeur -= 1,
            TokenTree::Punct(p) if p.as_char() == ',' && profondeur == 0 => {
                terminer(&mut segment);
                continue;
            }
            _ => {}
        }
        segment.push(jeton);
    }
    terminer(&mut segment);
    resultat
}

// ───────────────────────── 2. Macro attribut ─────────────────────────

/// `#[chronometre]` : mesure et affiche (sur stderr) la durée d'exécution de la fonction.
///
/// Le corps original est enveloppé dans une closure, de sorte que `return` et `?` fonctionnent.
/// Limite : fonctions non `async`.
#[proc_macro_attribute]
pub fn chronometre(_attributs: TokenStream, element: TokenStream) -> TokenStream {
    let mut jetons: Vec<TokenTree> = element.into_iter().collect();

    // Nom de la fonction : l'identifiant qui suit `fn`.
    let mut nom = String::from("fonction");
    for i in 0..jetons.len() {
        if let TokenTree::Ident(id) = &jetons[i] {
            if id.to_string() == "fn" {
                if let Some(TokenTree::Ident(n)) = jetons.get(i + 1) {
                    nom = n.to_string();
                }
                break;
            }
        }
    }

    // Le corps est le dernier jeton : un groupe entre accolades.
    let corps: Group = match jetons.pop() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => g,
        _ => panic!("#[chronometre] s'applique à une fonction avec un corps"),
    };

    let nouveau_corps: TokenStream = format!(
        "{{
            let __debut = ::std::time::Instant::now();
            let __resultat = (|| {corps})();
            ::std::eprintln!(\"[chronometre] {nom} : {{:?}}\", __debut.elapsed());
            __resultat
        }}"
    )
    .parse()
    .expect("code généré invalide");

    let mut sortie: TokenStream = jetons.into_iter().collect(); // signature intacte
    sortie.extend(nouveau_corps);
    sortie
}

// ───────────────────────── 3. Macro de type fonction ─────────────────────────

/// `carre_const!(12)` → le littéral `144i64`, calculé à la compilation.
#[proc_macro]
pub fn carre_const(entree: TokenStream) -> TokenStream {
    let texte = entree.to_string();
    let n: i64 = texte
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("carre_const! attend un entier littéral, reçu : {texte:?}"));
    format!("{}i64", n * n).parse().expect("littéral invalide")
}
