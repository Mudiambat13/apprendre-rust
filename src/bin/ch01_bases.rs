//! # Chapitre 1 — Les bases
//!
//! Variables, mutabilité, types scalaires, tuples, tableaux, fonctions.
//!
//! Lancer : `cargo run --bin ch01_bases`
//! Cours  : `docs/chapitres/01-bases.md`

/// Une constante : type obligatoire, évaluée à la compilation, `SCREAMING_SNAKE_CASE`.
const POINTS_MAX: u32 = 100_000;

fn main() {
    variables();
    types_scalaires();
    debordements();
    conversions();
    types_composes();
    fonctions();
}

fn variables() {
    println!("--- Variables ---");

    // Immuable par défaut.
    let x = 5;
    // `mut` rend la variable modifiable.
    let mut y = 10;
    y += x;
    println!("x = {x}, y = {y}, POINTS_MAX = {POINTS_MAX}");

    // Le shadowing crée une *nouvelle* variable (on peut même changer de type).
    let espaces = "   ";
    let espaces = espaces.len();
    println!("nombre d'espaces : {espaces}");

    // Un bloc est une expression : sa dernière ligne (sans `;`) est sa valeur.
    let z = {
        let a = 3;
        a * a + 1
    };
    println!("z = {z}");
}

fn types_scalaires() {
    println!("--- Types scalaires ---");

    let entier: i32 = -42; // i8 i16 i32 i64 i128 isize
    let positif: u8 = 255; // u8 u16 u32 u64 u128 usize
    let flottant: f64 = 1.618_033_988; // f32 ou f64
    let booleen: bool = true;
    let caractere: char = '🦀'; // un `char` fait 4 octets (valeur scalaire Unicode)

    println!("{entier} {positif} {flottant:.3} {booleen} {caractere}");
    println!("i32 : de {} à {}", i32::MIN, i32::MAX);
    println!("taille d'un char : {} octets", size_of::<char>());

    // Littéraux : séparateurs `_`, suffixes de type, bases.
    let hexa = 0xff;
    let octal = 0o77;
    let binaire = 0b1111_0000;
    let octet = b'A';
    let grand = 1_000_000_u64;
    println!("{hexa} {octal} {binaire} {octet} {grand}");
}

fn debordements() {
    println!("--- Débordements ---");
    // En debug : panic. En release : wrapping silencieux.
    // On choisit donc explicitement le comportement voulu :
    let n: u8 = 250;
    println!("checked    : {:?}", n.checked_add(10)); // None
    println!("wrapping   : {}", n.wrapping_add(10)); // 4
    println!("saturating : {}", n.saturating_add(10)); // 255
    println!("overflowing: {:?}", n.overflowing_add(10)); // (4, true)
}

fn conversions() {
    println!("--- Conversions ---");
    // Rust ne convertit JAMAIS implicitement entre types numériques.
    let petit: u8 = 200;
    let grand: u32 = petit as u32 + 100;
    println!("as (élargissement) : {grand}");

    // `as` tronque silencieusement : à utiliser en connaissance de cause.
    println!("300 as u8   = {}", 300_i32 as u8); // 44
    println!("-1 as u32   = {}", -1_i32 as u32); // 4294967295
    println!("3.99 as i32 = {}", 3.99_f64 as i32); // 3

    // `try_from` / `try_into` : conversion qui peut échouer proprement.
    println!("u8::try_from(300) = {:?}", u8::try_from(300_i32).is_err());
    let ok: Result<u8, _> = 42_i32.try_into();
    println!("42 → u8 = {ok:?}");
}

fn types_composes() {
    println!("--- Tuples et tableaux ---");

    let tuple: (i32, f64, char) = (500, 6.4, 'z');
    let (a, b, c) = tuple; // déstructuration
    println!("{a} {b} {c} | accès par index : {}", tuple.0);

    // Tableau : taille fixe connue à la compilation, alloué sur la pile.
    let mois = ["jan", "fév", "mar", "avr"];
    let zeros = [0u8; 5]; // [valeur; longueur]
    println!("{mois:?} {zeros:?} len={}", mois.len());

    // Les accès hors limites sont vérifiés → panic plutôt que comportement indéfini.
    // let oups = mois[10]; // ← panic à l'exécution (et erreur de compilation ici car constant)
    println!("get(10) = {:?}", mois.get(10)); // None : accès sûr
}

fn fonctions() {
    println!("--- Fonctions ---");
    println!("carre(7) = {}", carre(7));
    println!("7 est {}", parite(7));
    let (q, r) = division_euclidienne(17, 5);
    println!("17 = 5 × {q} + {r}");
}

/// La dernière expression (sans `;`) est la valeur de retour.
fn carre(n: i32) -> i32 {
    n * n
}

fn parite(n: i32) -> &'static str {
    if n % 2 == 0 { "pair" } else { "impair" }
}

/// Retourner plusieurs valeurs : un tuple.
fn division_euclidienne(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
