//! # Chapitre 3 — Ownership, emprunts et slices
//!
//! LE concept central de Rust : la gestion mémoire sans ramasse-miettes.
//!
//! Les trois règles :
//!   1. Chaque valeur a **un seul propriétaire**.
//!   2. Quand le propriétaire sort de portée, la valeur est **libérée** (`Drop`).
//!   3. On peut avoir soit **plusieurs `&T`**, soit **un seul `&mut T`** — jamais les deux.
//!
//! Lancer : `cargo run --bin ch03_ownership`

fn main() {
    deplacement();
    copie();
    emprunts();
    slices();
}

fn deplacement() {
    println!("--- Move ---");
    let s1 = String::from("bonjour"); // String : propriétaire d'un buffer sur le tas
    let s2 = s1; // le buffer change de propriétaire : s1 n'est plus utilisable
    // println!("{s1}"); // ← ERREUR : borrow of moved value `s1`
    println!("s2 = {s2}");

    // `clone()` fait une copie profonde (coûteuse, donc explicite).
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");

    // Passer une valeur à une fonction la déplace aussi.
    let nom = String::from("Alice");
    prend_possession(nom);
    // println!("{nom}"); // ← ERREUR : `nom` a été déplacé dans la fonction

    // On peut rendre la valeur… mais les emprunts sont plus simples (voir plus bas).
    let nom = String::from("Bob");
    let nom = prend_et_rend(nom);
    println!("récupéré : {nom}");
}

fn prend_possession(s: String) {
    println!("j'ai reçu {s}");
} // `s` est libéré ici

fn prend_et_rend(s: String) -> String {
    s
}

fn copie() {
    println!("--- Copy ---");
    // Les types simples, de taille fixe et sur la pile, implémentent `Copy` :
    // l'affectation les COPIE au lieu de les déplacer.
    let a = 5;
    let b = a;
    println!("a = {a}, b = {b}"); // les deux restent valides

    // Copy : entiers, flottants, bool, char, &T, tuples/tableaux de Copy…
    // Pas Copy : String, Vec<T>, Box<T>… (ils possèdent des ressources).
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 1, y: 2 };
    let q = p; // copie
    println!("{p:?} {q:?} ({} {})", p.x, q.y);
}

fn emprunts() {
    println!("--- Emprunts (références) ---");

    let texte = String::from("hello");

    // &T : emprunt partagé (lecture seule). Plusieurs possibles simultanément.
    let r1 = &texte;
    let r2 = &texte;
    println!("{r1} {r2} longueur = {}", longueur(&texte));

    // &mut T : emprunt exclusif (lecture + écriture). Un seul à la fois.
    let mut mot = String::from("rust");
    ajoute_exclamation(&mut mot);
    println!("{mot}");

    // NLL (Non-Lexical Lifetimes) : un emprunt dure jusqu'à sa DERNIÈRE utilisation.
    let mut v = vec![1, 2, 3];
    let premier = &v[0];
    println!("premier = {premier}"); // dernière utilisation de `premier`
    v.push(4); // OK : l'emprunt partagé est terminé
    println!("{v:?}");

    // Ce qui est refusé (garantie anti data-race et anti use-after-free) :
    //   let premier = &v[0];
    //   v.push(5);            // ERREUR : v est emprunté de façon immuable
    //   println!("{premier}");
}

#[allow(clippy::ptr_arg)] // volontaire ici : on montre un emprunt de String ; préférez &str
fn longueur(s: &String) -> usize {
    s.len()
}

fn ajoute_exclamation(s: &mut String) {
    s.push('!');
}

fn slices() {
    println!("--- Slices ---");
    // Une slice est une VUE (pointeur + longueur) sur une portion contiguë.
    let phrase = String::from("le langage rust est génial");
    let mot = premier_mot(&phrase);
    println!("premier mot : {mot}");

    // &str est le type des slices de String ; les littéraux sont des &'static str.
    let litteral: &str = "je suis un &str";
    println!("{}", premier_mot(litteral)); // `&str` accepte aussi bien &String que &str

    // Slices de tableaux/vecteurs.
    let nombres = [10, 20, 30, 40, 50];
    let milieu: &[i32] = &nombres[1..4];
    println!("{milieu:?} somme = {}", somme(milieu));
    println!("{:?} {:?}", &nombres[..2], &nombres[3..]);

    // ⚠ Les indices d'une &str sont des OCTETS, pas des caractères.
    let accents = "été";
    println!(
        "'{accents}' : {} octets, {} caractères",
        accents.len(),
        accents.chars().count()
    );
    // &accents[0..1] ferait paniquer : on couperait au milieu d'un 'é' (2 octets).
}

/// Règle d'or d'API : accepter `&str` plutôt que `&String`, `&[T]` plutôt que `&Vec<T>`.
fn premier_mot(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn somme(valeurs: &[i32]) -> i32 {
    valeurs.iter().sum()
}
