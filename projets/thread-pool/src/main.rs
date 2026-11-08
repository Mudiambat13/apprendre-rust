//! Démo : calculer des nombres premiers en parallèle avec notre `ThreadPool`.
//!
//! `cargo run --release -p thread-pool`

use std::sync::mpsc;
use std::time::Instant;

use thread_pool::ThreadPool;

fn est_premier(n: u64) -> bool {
    n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

/// Compte les nombres premiers dans [debut, fin).
fn compter_premiers(debut: u64, fin: u64) -> usize {
    (debut..fin).filter(|&n| est_premier(n)).count()
}

fn main() {
    const BORNE: u64 = 3_000_000;
    const MORCEAUX: u64 = 30;

    let nb_workers = std::thread::available_parallelism().map_or(4, |n| n.get());
    println!("pool de {nb_workers} workers, calcul des premiers < {BORNE}");

    // Séquentiel (référence)
    let t0 = Instant::now();
    let total_seq = compter_premiers(0, BORNE);
    println!("séquentiel : {total_seq} premiers en {:?}", t0.elapsed());

    // Parallèle
    let t0 = Instant::now();
    let pool = ThreadPool::new(nb_workers).expect("taille valide");
    let (tx, rx) = mpsc::channel();
    let taille = BORNE / MORCEAUX;
    for i in 0..MORCEAUX {
        let tx = tx.clone();
        pool.execute(move || {
            let (debut, fin) = (i * taille, (i + 1) * taille);
            tx.send(compter_premiers(debut, fin))
                .expect("récepteur vivant");
        });
    }
    drop(tx);
    let total_par: usize = rx.iter().sum();
    println!("parallèle  : {total_par} premiers en {:?}", t0.elapsed());

    assert_eq!(total_seq, total_par);
    println!("✔ les deux résultats sont identiques");
}
