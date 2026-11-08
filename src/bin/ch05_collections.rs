//! # Chapitre 5 — Collections : `Vec`, `String`, `HashMap`, …
//!
//! Lancer : `cargo run --bin ch05_collections`

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    vecteurs();
    chaines();
    hashmaps();
    autres_collections();
}

fn vecteurs() {
    println!("--- Vec<T> ---");
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    v.push(5);
    v.insert(0, 100);
    println!("{v:?} len={} cap>={}", v.len(), v.capacity());

    // Accès : `v[i]` panique si hors limites ; `get(i)` renvoie une Option.
    println!("{} {:?} {:?}", v[1], v.get(2), v.get(99));

    v.sort();
    v.dedup(); // retire les doublons CONSÉCUTIFS (d'où le tri préalable)
    v.retain(|&x| x != 100);
    println!("trié sans doublons : {v:?}");

    // Itérer : par référence, par référence mutable, ou en consommant.
    for x in &mut v {
        *x *= 10;
    }
    let total: i32 = v.iter().sum();
    println!("{v:?} total={total}");

    // Pré-allouer quand on connaît la taille : évite les réallocations.
    let mut grand = Vec::with_capacity(1_000);
    grand.extend(0..1_000);
    println!("cap={} len={}", grand.capacity(), grand.len());

    // Recherche binaire (sur tranche triée), découpage, concaténation.
    println!("{:?} {:?}", v.binary_search(&40), v.binary_search(&41));
    for morceau in [1, 2, 3, 4, 5].chunks(2) {
        print!("{morceau:?} ");
    }
    println!();
}

fn chaines() {
    println!("--- String et &str ---");
    // String = buffer UTF-8 possédé et extensible ; &str = vue en lecture seule.
    let mut s = String::from("Salut");
    s.push_str(", monde");
    s.push('!');
    println!("{s} ({} octets)", s.len());

    // Concaténation
    let a = String::from("Rust");
    let b = String::from("acé");
    let ab = a + &b; // `a` est déplacé ; `b` emprunté
    let c = format!("{ab}-{}-{}", 2024, "ok");
    println!("{c}");

    // UTF-8 : on ne peut PAS indexer par entier (s[0] n'existe pas).
    let mot = "नमस्ते"; // 6 chars, 18 octets
    println!("{} octets, {} chars", mot.len(), mot.chars().count());
    for (i, c) in "añb".char_indices() {
        print!("{i}:{c} ");
    }
    println!();

    // Méthodes utiles
    let phrase = "  Le Rust, c'est chouette  ";
    println!("{:?}", phrase.trim());
    println!("{:?}", phrase.to_uppercase());
    println!("{:?}", phrase.trim().split(", ").collect::<Vec<_>>());
    println!(
        "{} {}",
        phrase.contains("Rust"),
        phrase.replace("Rust", "Python")
    );
    println!(
        "{:?} {:?}",
        "42".parse::<i32>(),
        "abc".parse::<i32>().is_err()
    );
    println!(
        "{:?}",
        "a=1;b=2"
            .split(';')
            .map(|kv| kv.split_once('='))
            .collect::<Vec<_>>()
    );
}

fn hashmaps() {
    println!("--- HashMap<K, V> ---");
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("Alice".to_string(), 10);
    scores.insert("Bob".to_string(), 7);
    scores.insert("Alice".to_string(), 12); // remplace la valeur précédente

    println!("{:?} {:?}", scores.get("Alice"), scores.get("Zoé"));

    // L'API `entry` : « insère si absent, puis modifie » en une seule recherche.
    let texte = "le chat et le chien et le rat";
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for mot in texte.split_whitespace() {
        *freq.entry(mot).or_insert(0) += 1;
    }
    let mut tri: Vec<_> = freq.iter().collect();
    tri.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    println!("{tri:?}");

    // Construire depuis un itérateur de paires
    let carres: HashMap<u32, u32> = (1..=5).map(|n| (n, n * n)).collect();
    let mut cles: Vec<_> = carres.keys().copied().collect();
    cles.sort();
    println!("{cles:?} → 4² = {}", carres[&4]);

    // L'ordre d'itération d'un HashMap est ALÉATOIRE (protection contre le HashDoS).
    scores.remove("Bob");
    println!(
        "{} entrée(s) restante(s), contient Alice : {}",
        scores.len(),
        scores.contains_key("Alice")
    );
}

fn autres_collections() {
    println!("--- Autres collections ---");

    // HashSet : ensemble sans doublons, opérations ensemblistes.
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
    let mut inter: Vec<_> = a.intersection(&b).copied().collect();
    inter.sort();
    println!("intersection = {inter:?}");

    // BTreeMap / BTreeSet : triés par clé, parcours ordonné, requêtes par intervalle.
    let mut notes = BTreeMap::new();
    notes.insert(3, "trois");
    notes.insert(1, "un");
    notes.insert(2, "deux");
    println!(
        "{notes:?} | plage 2.. = {:?}",
        notes.range(2..).collect::<Vec<_>>()
    );
    let ensemble: BTreeSet<_> = "chocolat".chars().collect();
    println!("{ensemble:?}");

    // VecDeque : file double (ajout/retrait O(1) aux deux extrémités).
    let mut file = VecDeque::new();
    file.push_back("premier");
    file.push_back("deuxième");
    file.push_front("prioritaire");
    let premier = file.pop_front();
    println!("pop_front = {premier:?} ; reste = {file:?}");

    // BinaryHeap : tas max (file de priorité).
    let mut tas = BinaryHeap::from([5, 1, 8, 3]);
    tas.push(7);
    print!("tas (décroissant) : ");
    while let Some(x) = tas.pop() {
        print!("{x} ");
    }
    println!();
}
