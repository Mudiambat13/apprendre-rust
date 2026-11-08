//! Un pool de threads : N workers créés une fois, qui se partagent une file de tâches.
//!
//! Architecture :
//!
//! ```text
//!  ThreadPool::execute(job) ──► Sender ═════► Arc<Mutex<Receiver>> ◄── worker 0
//!                                                                  ◄── worker 1
//!                                                                  ◄── worker N-1
//! ```
//!
//! - `Sender` : côté producteur, détenu par le `ThreadPool`.
//! - `Receiver` : un seul, partagé par tous les workers via `Arc<Mutex<…>>`.
//! - Arrêt propre : on détruit le `Sender` ; `recv()` renvoie alors `Err` et chaque worker
//!   sort de sa boucle ; `Drop` fait `join` sur tous les threads.

use std::error::Error;
use std::fmt;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

type Tache = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug, PartialEq, Eq)]
pub struct ErreurPool;

impl fmt::Display for ErreurPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "un pool doit contenir au moins un thread")
    }
}

impl Error for ErreurPool {}

pub struct ThreadPool {
    workers: Vec<Worker>,
    emetteur: Option<mpsc::Sender<Tache>>,
}

struct Worker {
    id: usize,
    fil: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Crée un pool de `taille` threads.
    ///
    /// # Erreurs
    /// [`ErreurPool`] si `taille == 0`.
    pub fn new(taille: usize) -> Result<ThreadPool, ErreurPool> {
        if taille == 0 {
            return Err(ErreurPool);
        }
        let (emetteur, recepteur) = mpsc::channel::<Tache>();
        let recepteur = Arc::new(Mutex::new(recepteur));

        let workers = (0..taille)
            .map(|id| Worker::new(id, Arc::clone(&recepteur)))
            .collect();
        Ok(ThreadPool {
            workers,
            emetteur: Some(emetteur),
        })
    }

    /// Soumet une tâche. Elle sera exécutée par le premier worker libre.
    pub fn execute<F>(&self, tache: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.emetteur
            .as_ref()
            .expect("le pool est en cours d'arrêt")
            .send(Box::new(tache))
            .expect("tous les workers se sont arrêtés");
    }

    pub fn nombre_de_workers(&self) -> usize {
        self.workers.len()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 1. Fermer le canal : plus aucune tâche ne sera envoyée.
        drop(self.emetteur.take());
        // 2. Attendre que chaque worker ait fini ses tâches en cours/en file.
        for worker in &mut self.workers {
            if let Some(fil) = worker.fil.take() {
                let _ = fil.join();
                let _ = worker.id; // (on pourrait journaliser « worker {id} arrêté »)
            }
        }
    }
}

impl Worker {
    fn new(id: usize, recepteur: Arc<Mutex<mpsc::Receiver<Tache>>>) -> Worker {
        let fil = thread::spawn(move || {
            loop {
                // Le verrou n'est tenu que pendant `recv()` : il est relâché AVANT d'exécuter
                // la tâche (le guard est un temporaire détruit à la fin de cette instruction).
                let message = recepteur.lock().expect("mutex empoisonné").recv();
                match message {
                    Ok(tache) => tache(),
                    Err(_) => break, // canal fermé : on s'arrête
                }
            }
        });
        Worker { id, fil: Some(fil) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn taille_zero_refusee() {
        assert_eq!(ThreadPool::new(0).err(), Some(ErreurPool));
    }

    #[test]
    fn toutes_les_taches_sont_executees() {
        let compteur = Arc::new(AtomicUsize::new(0));
        {
            let pool = ThreadPool::new(4).unwrap();
            assert_eq!(pool.nombre_de_workers(), 4);
            for _ in 0..100 {
                let compteur = Arc::clone(&compteur);
                pool.execute(move || {
                    compteur.fetch_add(1, Ordering::SeqCst);
                });
            }
        } // le Drop attend la fin de toutes les tâches
        assert_eq!(compteur.load(Ordering::SeqCst), 100);
    }

    #[test]
    fn resultats_via_un_canal() {
        let (tx, rx) = mpsc::channel();
        let pool = ThreadPool::new(3).unwrap();
        for n in 1..=5u64 {
            let tx = tx.clone();
            pool.execute(move || tx.send((n, n * n)).unwrap());
        }
        drop(tx);
        let mut resultats: Vec<_> = rx.iter().collect();
        resultats.sort();
        assert_eq!(resultats, vec![(1, 1), (2, 4), (3, 9), (4, 16), (5, 25)]);
    }

    #[test]
    fn un_panic_dans_une_tache_ne_bloque_pas_le_pool() {
        let compteur = Arc::new(AtomicUsize::new(0));
        {
            let pool = ThreadPool::new(2).unwrap();
            pool.execute(|| panic!("tâche défaillante (attendu dans ce test)"));
            for _ in 0..10 {
                let compteur = Arc::clone(&compteur);
                pool.execute(move || {
                    compteur.fetch_add(1, Ordering::SeqCst);
                });
            }
        }
        // Un worker est mort ; l'autre a traité toutes les tâches restantes.
        assert_eq!(compteur.load(Ordering::SeqCst), 10);
    }
}
