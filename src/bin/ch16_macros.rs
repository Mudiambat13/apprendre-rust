//! # Chapitre 16 — Macros déclaratives (`macro_rules!`)
//!
//! Une macro est du code qui **génère du code** à la compilation, avant la vérification des types.
//! Une règle = un *motif* (pattern) → un *gabarit* (template).
//!
//! Fragments courants : `expr`, `ident`, `ty`, `tt` (token tree), `literal`, `pat`, `stmt`,
//! `block`, `item`, `path`, `lifetime`, `vis`.
//! Répétitions : `$( … ),*` (zéro ou plus), `$( … );+` (un ou plus), `$( … )?` (optionnel).
//!
//! Lancer : `cargo run --bin ch16_macros`

#![allow(dead_code)]

use std::collections::HashMap;

/// Construit un `HashMap` : `hashmap!{ "a" => 1, "b" => 2 }`.
macro_rules! hashmap {
    // `$(,)?` autorise une virgule finale optionnelle.
    ($($cle:expr => $valeur:expr),* $(,)?) => {{
        let mut m = ::std::collections::HashMap::new();
        $( m.insert($cle, $valeur); )*
        m
    }};
}

/// Récursion : `max!(1, 5, 3)`.
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($reste:expr),+) => {{
        let a = $x;               // évalué UNE seule fois (évite les effets de bord doublés)
        let b = max!($($reste),+);
        if a > b { a } else { b }
    }};
}

/// Compte les éléments à la compilation (technique du « tt muncher »).
macro_rules! compter {
    () => (0usize);
    ($_tete:tt $($queue:tt)*) => (1usize + compter!($($queue)*));
}

/// Génère une struct ET ses méthodes.
macro_rules! creer_struct {
    ($nom:ident { $($champ:ident : $type:ty),* $(,)? }) => {
        #[derive(Debug, Default)]
        struct $nom {
            $($champ: $type),*
        }

        impl $nom {
            fn noms_des_champs() -> Vec<&'static str> {
                vec![$(stringify!($champ)),*]
            }
        }
    };
}

creer_struct!(Utilisateur {
    nom: String,
    age: u32,
    actif: bool
});

/// Implémente un trait pour plusieurs types d'un coup.
trait Doubler {
    fn doubler(&self) -> Self;
}
macro_rules! impl_doubler {
    ($($t:ty),+ $(,)?) => {
        $( impl Doubler for $t { fn doubler(&self) -> Self { *self * 2 as $t } } )+
    };
}
impl_doubler!(i32, u64, f64);

/// Une macro peut avoir plusieurs « bras » avec des motifs différents.
macro_rules! calcul {
    (ajouter $a:expr, $b:expr) => {
        $a + $b
    };
    (multiplier $a:expr, $b:expr) => {
        $a * $b
    };
    (negatif $a:expr) => {
        -$a
    };
}

/// Test-driven : une macro pour générer des tests paramétrés (à utiliser dans `#[cfg(test)]`).
macro_rules! tests_carres {
    ($($nom:ident : $entree:expr => $attendu:expr),* $(,)?) => {
        #[cfg(test)]
        mod tests_carres {
            $( #[test] fn $nom() { assert_eq!($entree * $entree, $attendu); } )*
        }
    };
}
tests_carres! {
    carre_de_deux: 2 => 4,
    carre_de_dix: 10 => 100,
}

/// Hygiène : les variables locales d'une macro ne « fuient » pas dans le code appelant.
macro_rules! hygiene {
    ($e:expr) => {{
        let x = 100; // ce `x` n'est PAS le `x` de l'appelant
        $e + x
    }};
}

fn main() {
    println!("--- hashmap! ---");
    let m: HashMap<&str, i32> = hashmap! { "un" => 1, "deux" => 2, };
    let mut paires: Vec<_> = m.iter().collect();
    paires.sort();
    println!("{paires:?}");

    println!("--- max! / compter! ---");
    println!("max = {}", max!(3, 9, 4, 1));
    println!("compter = {}", compter!(a b c d e));

    println!("--- creer_struct! ---");
    let u = Utilisateur {
        nom: "Alice".into(),
        age: 30,
        actif: true,
    };
    println!("{u:?} champs = {:?}", Utilisateur::noms_des_champs());

    println!("--- impl_doubler! ---");
    println!(
        "{} {} {}",
        21_i32.doubler(),
        4_u64.doubler(),
        1.5_f64.doubler()
    );

    println!("--- plusieurs bras ---");
    println!(
        "{} {} {}",
        calcul!(ajouter 2, 3),
        calcul!(multiplier 4, 5),
        calcul!(negatif 7)
    );

    println!("--- hygiène ---");
    let x = 1;
    println!("{}", hygiene!(x)); // 1 + 100 = 101, pas 200

    println!("--- macros standard utiles ---");
    println!("{}", stringify!(a + b * 2)); // le code source, en texte
    println!("{}", concat!("v", 1, ".", 2));
    println!("fichier={} ligne={}", file!(), line!());
    println!("{}", matches!(Some(3), Some(n) if n > 2));
    #[allow(clippy::useless_vec)]
    let v = vec![1, 2, 3]; // `vec!` est elle-même une macro_rules!
    let _ = dbg!(v.len()); // affiche fichier:ligne = valeur sur stderr, et la renvoie
    assert!(v.len() == 3, "longueur inattendue : {}", v.len());
    debug_assert_eq!(v[0], 1);
    // Pour des macros plus puissantes (dériver des traits, parser des attributs) :
    // macros procédurales → chapitre 20.
}
