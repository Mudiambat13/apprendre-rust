# Chapitre 13 — Concurrence

> 🟠 Avancé · Code : [`src/bin/ch13_concurrence.rs`](../../src/bin/ch13_concurrence.rs) · projet [`thread-pool`](../../projets/thread-pool)

## Objectifs

Écrire du code parallèle où les *data races* sont **impossibles à la compilation**.

## Le contrat : `Send` et `Sync`

- `T: Send` — `T` peut être **déplacé** vers un autre thread.
- `T: Sync` — `&T` peut être **partagé** entre threads (⟺ `&T: Send`).
- Ces traits sont **automatiques** : le compilateur les déduit des champs. `Rc` n'est ni `Send` ni `Sync` ; `Arc<Mutex<T>>` l'est.

## Outils

| Besoin | Outil |
|:--|:--|
| Lancer un thread | `thread::spawn(move \|\| …)` → `JoinHandle` (`join()` récupère le résultat ou la panique) |
| Emprunter des variables locales | `thread::scope(\|s\| { s.spawn(…); })` : pas de `'static` ni `Arc` |
| Envoyer des messages | `mpsc::channel()` (N producteurs → 1 consommateur) ; `drop(tx)` pour fermer |
| Partager un état mutable | `Arc<Mutex<T>>` ; lecture fréquente : `Arc<RwLock<T>>` |
| Attendre une condition | `Condvar` (toujours dans un `while`, à cause des réveils parasites) |
| Compteurs/drapeaux | `AtomicUsize`, `AtomicBool`… |
| Parallélisme de données | crate **`rayon`** : `v.par_iter().sum()` |

> « Ne communiquez pas en partageant la mémoire ; partagez la mémoire en communiquant. » Préférez les canaux quand c'est naturel.

## Ordres mémoire (atomiques)

`Relaxed` (aucun ordre, ok pour un compteur isolé) · `Acquire`/`Release` (publier des données via un drapeau) · `SeqCst` (ordre total, choix sûr par défaut). Pour aller loin : *Rust Atomics and Locks*.

## Dangers que Rust **ne** supprime **pas**

- **Deadlock** : deux verrous pris dans un ordre différent. Règle : ordre global, ou un seul verrou à la fois.
- **Poisoning** : un thread qui panique en tenant un `Mutex` l'« empoisonne » (`lock()` renvoie `Err`).
- **Contention** : trop de threads sur un même verrou → plus lent que séquentiel. Mesurez.
- **Logique** (conditions de course de haut niveau) : l'atomicité d'une opération ≠ celle d'une séquence.

## À retenir

- Le verrou se relâche à la destruction du *guard* : gardez sa portée courte.
- Ne tenez jamais un verrou pendant une opération longue ou bloquante.

## Exercices

1. Niveau 3, ex. 3.1 (somme parallèle) et 3.3 (compteur atomique).
2. Provoquez un deadlock volontaire, puis corrigez-le.
3. Lisez `thread-pool/src/lib.rs` : pourquoi le verrou est-il relâché *avant* d'exécuter la tâche ?

## Pour aller plus loin

[The Book ch. 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html) · [`std::sync`](https://doc.rust-lang.org/std/sync/) · `rayon`, `crossbeam` · [*Rust Atomics and Locks*](https://marabos.nl/atomics/)
