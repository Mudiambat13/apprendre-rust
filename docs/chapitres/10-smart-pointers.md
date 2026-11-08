# Chapitre 10 — Pointeurs intelligents

> 🟡 Intermédiaire · Code : [`src/bin/ch10_smart_pointers.rs`](../../src/bin/ch10_smart_pointers.rs)

## Objectifs

Savoir quel pointeur utiliser selon le besoin : tas, partage, mutabilité intérieure.

## Panorama

| Type | Propriété | Mutabilité | Threads | Usage typique |
|:--|:--|:--|:--|:--|
| `Box<T>` | unique, sur le tas | selon le binding | ✔ si `T: Send` | types récursifs, `dyn Trait`, gros objets |
| `Rc<T>` | partagée (compteur) | lecture seule | ✘ | graphes, données partagées mono-thread |
| `Arc<T>` | partagée (compteur atomique) | lecture seule | ✔ | idem, multi-threads |
| `Cell<T>` | — | `get`/`set` (types `Copy`) | ✘ | compteur interne |
| `RefCell<T>` | — | emprunts vérifiés à l'**exécution** | ✘ | mutabilité intérieure |
| `Mutex<T>` / `RwLock<T>` | — | verrou | ✔ | état partagé entre threads |
| `Weak<T>` | non-propriétaire | — | selon `Rc`/`Arc` | parent→enfant, caches, casser les cycles |

Combinaisons classiques : `Rc<RefCell<T>>` (mono-thread) et `Arc<Mutex<T>>` (multi-threads).

## `Deref` et `Drop`

- `Deref` permet `*x` et la **coercition déréférencante** (`&Box<String>` → `&str` automatiquement).
- `Drop` s'exécute à la sortie de portée (RAII) : fichiers fermés, verrous relâchés, mémoire libérée. Ordre : variables en sens **inverse** de déclaration ; champs d'une struct dans l'ordre de déclaration. `drop(x)` libère à l'avance.

## Mutabilité intérieure

`RefCell` déplace la vérification « un seul `&mut` ou plusieurs `&` » de la compilation à l'exécution : une violation **panique**. À utiliser quand la structure logique est sûre mais que le compilateur ne peut pas le prouver (mocks, graphes). `try_borrow_mut()` renvoie un `Result` au lieu de paniquer.

## Cycles de références

`Rc` + `Rc` en boucle = fuite mémoire (compteurs jamais à zéro). Solution : le lien « retour » est un `Weak` ; on y accède via `upgrade() -> Option<Rc<T>>`.

## À retenir

- Commencez par `Box`/valeurs possédées ; passez à `Rc`/`Arc` seulement si plusieurs propriétaires sont réellement nécessaires.
- `RefCell` est un signal : « je contourne le borrow checker à mes risques d'exécution ».

## Exercices

1. Liste chaînée immuable avec `Box` (niveau 2, ex. 2.6).
2. Arbre avec enfants (`Rc`) et parent (`Weak`) : affichez `strong_count`/`weak_count`.
3. Implémentez `Drop` pour un type qui journalise sa destruction (niveau 4, ex. 4.3).

## Pour aller plus loin

[The Book ch. 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) · [`std::rc`](https://doc.rust-lang.org/std/rc/) · [`std::cell`](https://doc.rust-lang.org/std/cell/)
