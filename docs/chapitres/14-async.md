# Chapitre 14 — Programmation asynchrone

> 🟠 Avancé · Code : [`src/bin/ch14_async.rs`](../../src/bin/ch14_async.rs) (un mini-exécuteur écrit à la main)

## Pourquoi `async` ?

Un thread par connexion ne passe pas à l'échelle (mémoire, changements de contexte). L'asynchrone permet à **quelques threads** de gérer des **milliers de tâches** qui passent leur temps à attendre (réseau, disque).

| | Threads OS | `async` |
|:--|:--|:--|
| Idéal pour | calcul (CPU-bound) | attentes d'E/S (I/O-bound) |
| Coût par tâche | ~Mo de pile | quelques centaines d'octets |
| Ordonnancement | préemptif (OS) | **coopératif** (aux `.await`) |

## Le modèle

- `async fn` renvoie un **`Future`** : une machine à états **paresseuse** (rien ne s'exécute avant d'être *poll*é).
- `Future::poll` renvoie `Poll::Ready(v)` ou `Poll::Pending` (en ayant enregistré un `Waker` à réveiller quand l'événement arrive).
- `.await` suspend la fonction jusqu'à ce que le futur soit prêt.
- Un **exécuteur** (*runtime*) sonde les futurs. **La bibliothèque standard n'en fournit pas** : on choisit `tokio` (le plus répandu), `async-std`, `smol`…

Le programme du chapitre construit `block_on` et un exécuteur multitâche avec la seule `std`, pour démystifier le mécanisme.

## Avec Tokio (en pratique)

```toml
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(telecharger("a"), telecharger("b"));   // concurrent
    let t = tokio::spawn(async { 42 });                              // tâche
    println!("{a:?} {b:?} {}", t.await.unwrap());
}
```

## Règles d'or

1. **Ne bloquez jamais** dans une tâche async (`thread::sleep`, gros calcul, I/O synchrone) : cela gèle le thread de l'exécuteur. Utilisez `tokio::time::sleep`, `spawn_blocking`.
2. Tenir un `std::sync::Mutex` à travers un `.await` est un piège : préférez `tokio::sync::Mutex` ou réduisez la portée.
3. Un futur **abandonné** (dropped) est **annulé** à son prochain point d'attente : attention à l'*annulation* (cancel safety).
4. `async` est « contaminant » : un appelant d'`async fn` doit lui-même être async (ou utiliser `block_on`).

## `Pin`

Un futur généré peut se référencer lui-même ; le déplacer en mémoire invaliderait ces pointeurs. `Pin<&mut Self>` garantit qu'il ne bouge plus (chapitre 21).

## À retenir

- Async ≠ parallèle : c'est de la **concurrence** ; combinez avec un exécuteur multi-thread pour le parallélisme.
- Pour du CPU lourd : threads / `rayon`, pas `async`.

## Exercices

1. Ajoutez au mini-exécuteur une tâche qui utilise `Minuteur`.
2. Avec Tokio : téléchargez 3 URLs en parallèle avec `reqwest` et `join!`.
3. Comparez 10 000 tâches `tokio::spawn` à 10 000 `thread::spawn` (mémoire, temps).

## Pour aller plus loin

[Async Book](https://rust-lang.github.io/async-book/) · [Tokio tutorial](https://tokio.rs/tokio/tutorial) · *Async: What is blocking?* (Alice Ryhl)
