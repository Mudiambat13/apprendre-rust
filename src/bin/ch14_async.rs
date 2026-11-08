//! # Chapitre 14 — Programmation asynchrone, de l'intérieur
//!
//! Rust fournit le trait `Future` et la syntaxe `async/await`, mais **pas d'exécuteur** :
//! on utilise normalement `tokio`, `async-std` ou `smol`. Ici nous écrivons nous-mêmes un
//! mini-exécuteur avec la seule bibliothèque standard, pour comprendre ce qui se passe vraiment.
//!
//! Un `Future` est une machine à états : `poll()` avance autant que possible puis renvoie
//!   - `Poll::Ready(valeur)` : terminé ;
//!   - `Poll::Pending`       : « réveille-moi plus tard » (via le `Waker` du `Context`).
//!
//! Lancer : `cargo run --bin ch14_async`

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

// ───────────────────── 1. Un exécuteur minimal : block_on ─────────────────────

/// Le waker réveille (`unpark`) le thread qui attend le futur.
struct ReveilThread(Thread);

impl Wake for ReveilThread {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Exécute un futur jusqu'au bout sur le thread courant.
fn block_on<F: Future>(futur: F) -> F::Output {
    let mut futur = Box::pin(futur); // on « épingle » le futur : il ne doit plus bouger en mémoire
    let waker = Waker::from(Arc::new(ReveilThread(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        match futur.as_mut().poll(&mut cx) {
            Poll::Ready(valeur) => return valeur,
            Poll::Pending => thread::park(), // dort jusqu'à ce que quelqu'un appelle `wake`
        }
    }
}

// ───────────────────── 2. Écrire un Future à la main ─────────────────────

/// Compte à rebours : retourne `Pending` N fois avant d'être prêt.
struct CompteARebours(u32);

impl Future for CompteARebours {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 == 0 {
            Poll::Ready("décollage !")
        } else {
            println!("    T-{}", self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref(); // « repasse me voir tout de suite »
            Poll::Pending
        }
    }
}

// ───────────────────── 3. Un vrai timer qui réveille depuis un autre thread ─────────────────────

struct EtatPartage {
    termine: bool,
    waker: Option<Waker>,
}

struct Minuteur {
    etat: Arc<Mutex<EtatPartage>>,
}

impl Minuteur {
    fn new(duree: Duration) -> Self {
        let etat = Arc::new(Mutex::new(EtatPartage {
            termine: false,
            waker: None,
        }));
        let etat_thread = Arc::clone(&etat);
        thread::spawn(move || {
            thread::sleep(duree);
            let mut e = etat_thread.lock().unwrap();
            e.termine = true;
            if let Some(waker) = e.waker.take() {
                waker.wake(); // réveille l'exécuteur
            }
        });
        Minuteur { etat }
    }
}

impl Future for Minuteur {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut e = self.etat.lock().unwrap();
        if e.termine {
            Poll::Ready(())
        } else {
            // On (re)stocke le waker : l'exécuteur a pu changer entre deux polls.
            e.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// ───────────────────── 4. async/await : le compilateur écrit la machine à états ─────────────────────

async fn additionner_apres(ms: u64, a: i32, b: i32) -> i32 {
    Minuteur::new(Duration::from_millis(ms)).await;
    a + b
}

async fn scenario() -> i32 {
    println!("  scénario : début");
    let x = additionner_apres(30, 1, 2).await; // séquentiel : 30 ms
    let y = additionner_apres(30, 10, 20).await; // puis encore 30 ms
    println!("  scénario : x = {x}, y = {y}");
    x + y
}

// ───────────────────── 5. Concurrence coopérative : exécuteur multitâche ─────────────────────

/// Rend la main à l'exécuteur une fois, puis se termine.
struct CederLaMain(bool);

impl Future for CederLaMain {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn travailleur(nom: &'static str, etapes: u32) {
    for i in 1..=etapes {
        println!("    {nom} : étape {i}/{etapes}");
        CederLaMain(false).await; // point de suspension : les autres tâches peuvent avancer
    }
    println!("    {nom} : terminé");
}

type Tache = Pin<Box<dyn Future<Output = ()>>>;

/// Exécuteur « round-robin » mono-thread : sonde chaque tâche à tour de rôle.
fn executer_tout(mut taches: Vec<Tache>) {
    let mut cx = Context::from_waker(Waker::noop()); // on sonde tout en boucle : pas besoin de waker
    while !taches.is_empty() {
        // `retain_mut` garde seulement les tâches encore `Pending`.
        taches.retain_mut(|t| t.as_mut().poll(&mut cx).is_pending());
    }
}

fn main() {
    println!("--- 1. Un futur écrit à la main ---");
    println!("  → {}", block_on(CompteARebours(3)));

    println!("--- 2. async/await + minuteur ---");
    let debut = Instant::now();
    let total = block_on(scenario());
    println!(
        "  total = {total} en ≈ {} ms (séquentiel : 2 × 30 ms)",
        debut.elapsed().as_millis()
    );

    println!("--- 3. Plusieurs tâches entrelacées ---");
    let taches: Vec<Tache> = vec![
        Box::pin(travailleur("A", 3)),
        Box::pin(travailleur("B", 2)),
        Box::pin(travailleur("C", 1)),
    ];
    executer_tout(taches);

    println!();
    println!("Pour du vrai asynchrone en production : tokio, async-std ou smol.");
    println!("Voir docs/chapitres/14-async.md pour la suite.");
}
