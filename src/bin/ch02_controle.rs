//! # Chapitre 2 — Flux de contrôle
//!
//! `if`, `loop`, `while`, `for`, `match`, `if let`, `while let`.
//!
//! Lancer : `cargo run --bin ch02_controle`

fn main() {
    conditions();
    boucles();
    filtrage_par_motif();
    fizzbuzz();
}

fn conditions() {
    println!("--- if est une expression ---");
    let temperature = 23;
    // La condition doit être un `bool` (pas de « truthy » comme en C ou JS).
    let ressenti = if temperature > 30 {
        "chaud"
    } else if temperature > 15 {
        "agréable"
    } else {
        "frais"
    };
    println!("{temperature}°C → {ressenti}");
}

fn boucles() {
    println!("--- Boucles ---");

    // `loop` : boucle infinie ; `break valeur` renvoie une valeur.
    let mut compteur = 0;
    let resultat = loop {
        compteur += 1;
        if compteur == 10 {
            break compteur * 2;
        }
    };
    println!("loop → {resultat}");

    // `while`
    let mut n = 3;
    while n > 0 {
        print!("{n}… ");
        n -= 1;
    }
    println!("décollage !");

    // `for` sur un intervalle : `a..b` exclut b, `a..=b` l'inclut.
    let somme: u32 = (1..=100).sum();
    println!("1+2+…+100 = {somme}");

    // Itération idiomatique : jamais d'index manuel si on peut l'éviter.
    let fruits = ["pomme", "poire", "cerise"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  {i}: {fruit}");
    }
    for n in (1..=3).rev() {
        print!("{n} ");
    }
    println!();

    // Étiquettes pour sortir de boucles imbriquées.
    'externe: for i in 1..10 {
        for j in 1..10 {
            if i * j == 42 {
                println!("trouvé : {i} × {j} = 42");
                break 'externe;
            }
        }
    }
}

fn filtrage_par_motif() {
    println!("--- match ---");

    // `match` est exhaustif : le compilateur vérifie que tous les cas sont traités.
    for n in [0, 1, 7, 15, 42, 100] {
        let description = match n {
            0 => "zéro".to_string(),
            1..=3 => "très petit".to_string(),
            4..=9 => "petit".to_string(),
            x if x % 2 == 0 => format!("{x} est pair (garde)"),
            x => format!("{x} est impair"),
        };
        println!("{n:>3} → {description}");
    }

    // Déstructuration de tuples dans un match.
    let point = (3, -3);
    match point {
        (0, 0) => println!("origine"),
        (x, 0) | (0, x) => println!("sur un axe à {x}"),
        (x, y) if x == -y => println!("sur la 2e diagonale"),
        _ => println!("ailleurs"),
    }

    // `if let` : quand on ne s'intéresse qu'à UN motif.
    let config: Option<u16> = Some(8080);
    if let Some(port) = config {
        println!("port configuré : {port}");
    }

    // `while let` : boucle tant que le motif correspond.
    let mut pile = vec![1, 2, 3];
    while let Some(sommet) = pile.pop() {
        print!("dépile {sommet}; ");
    }
    println!();

    // `let … else` : sortir tôt si le motif ne correspond pas.
    let texte = "42";
    let Ok(nombre) = texte.parse::<i32>() else {
        println!("pas un nombre");
        return;
    };
    println!("nombre = {nombre}");
}

fn fizzbuzz() {
    println!("--- FizzBuzz avec match sur un tuple ---");
    for n in 1..=15 {
        let s = match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => n.to_string(),
        };
        print!("{s} ");
    }
    println!();
}
