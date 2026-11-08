//! # Chapitre 13 — Concurrence « sans peur »
//!
//! Le système de types (traits `Send` et `Sync`) élimine les *data races* à la compilation.
//!
//! Lancer : `cargo run --bin ch13_concurrence`

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    threads_de_base();
    threads_scopes();
    canaux();
    etat_partage();
    atomiques();
    condvar();
}

fn threads_de_base() {
    println!("--- thread::spawn + join ---");
    let v = vec![1, 2, 3];

    // `move` : le thread est autonome → il doit POSSÉDER ce qu'il utilise ('static).
    let handle = thread::spawn(move || {
        println!("  thread : v = {v:?}");
        v.iter().sum::<i32>()
    });
    // `join` attend la fin et renvoie le résultat (ou Err si le thread a paniqué).
    let somme = handle.join().expect("le thread a paniqué");
    println!("somme = {somme}");

    // Un panic dans un thread n'abat pas les autres : on le détecte via join().
    let echec = thread::spawn(|| panic!("boum")).join();
    println!("thread paniqué ? {}", echec.is_err());
}

fn threads_scopes() {
    println!("--- thread::scope : emprunter sans 'static ---");
    let mut donnees = vec![1, 2, 3, 4, 5, 6];
    let message = String::from("emprunté");

    // Les threads « scopés » sont garantis terminés avant la sortie du scope,
    // donc ils peuvent emprunter des variables locales.
    thread::scope(|s| {
        let (gauche, droite) = donnees.split_at_mut(3);
        s.spawn(|| gauche.iter_mut().for_each(|x| *x *= 10));
        s.spawn(|| droite.iter_mut().for_each(|x| *x *= 100));
        s.spawn(|| println!("  {message}"));
    });
    println!("{donnees:?}");
}

fn canaux() {
    println!("--- mpsc : multiple producers, single consumer ---");
    let (tx, rx) = mpsc::channel();

    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 0..2 {
                tx.send(format!("worker {id} → message {i}")).unwrap();
                thread::sleep(Duration::from_millis(5));
            }
        });
    }
    drop(tx); // IMPORTANT : sans ça, `rx` n'est jamais « fermé » et la boucle ne finit pas.

    let mut recus: Vec<String> = rx.iter().collect();
    recus.sort();
    for m in &recus {
        println!("  {m}");
    }
}

fn etat_partage() {
    println!("--- Arc<Mutex<T>> et RwLock ---");

    // Arc = Rc thread-safe (compteur atomique). Mutex = exclusion mutuelle.
    let compteur = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let compteur = Arc::clone(&compteur);
            thread::spawn(move || {
                for _ in 0..1_000 {
                    // Le verrou est libéré à la fin de l'instruction (le guard est un temporaire).
                    *compteur.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("compteur = {} (attendu 8000)", *compteur.lock().unwrap());

    // RwLock : plusieurs lecteurs OU un seul écrivain.
    let config = Arc::new(RwLock::new(String::from("v1")));
    {
        let lecteur1 = config.read().unwrap();
        let lecteur2 = config.read().unwrap();
        println!("lecteurs simultanés : {lecteur1} {lecteur2}");
    }
    *config.write().unwrap() = String::from("v2");
    println!("après écriture : {}", config.read().unwrap());

    // Poisoning : si un thread panique en tenant le verrou, le Mutex est « empoisonné ».
    let m = Arc::new(Mutex::new(1));
    let m2 = Arc::clone(&m);
    let _ = thread::spawn(move || {
        let _garde = m2.lock().unwrap();
        panic!("panic avec le verrou");
    })
    .join();
    println!("mutex empoisonné ? {}", m.is_poisoned());
    let valeur = *m.lock().unwrap_or_else(|e| e.into_inner()); // on récupère quand même la donnée
    println!("valeur récupérée = {valeur}");

    // ⚠ Deadlock : deux threads qui prennent A puis B / B puis A. Règle : un ordre global
    //   des verrous, ou ne jamais tenir deux verrous à la fois.
}

fn atomiques() {
    println!("--- Atomiques ---");
    // Pour un simple compteur/drapeau, un atomique est plus léger qu'un Mutex.
    let compteur = Arc::new(AtomicUsize::new(0));
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..1_000 {
                    compteur.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
    println!("compteur atomique = {}", compteur.load(Ordering::SeqCst));
    // `Relaxed` : pas d'ordre entre variables (OK pour un compteur isolé).
    // `Acquire`/`Release` : synchronisent des données autour d'un drapeau.
    // `SeqCst` : le plus fort, ordre total ; choix sûr par défaut quand on débute.
}

fn condvar() {
    println!("--- Condvar : attendre une condition ---");
    let paire = Arc::new((Mutex::new(false), Condvar::new()));
    let paire2 = Arc::clone(&paire);

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        let (verrou, cv) = &*paire2;
        *verrou.lock().unwrap() = true;
        cv.notify_one();
    });

    let (verrou, cv) = &*paire;
    let mut pret = verrou.lock().unwrap();
    // `while` obligatoire : les réveils « parasites » (spurious wakeups) existent.
    while !*pret {
        pret = cv.wait(pret).unwrap();
    }
    println!("condition remplie : {}", *pret);
}
