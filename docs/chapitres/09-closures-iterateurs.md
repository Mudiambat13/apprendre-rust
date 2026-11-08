# Chapitre 9 — Closures et itérateurs

> 🟡 Intermédiaire · Code : [`src/bin/ch09_closures_iterateurs.rs`](../../src/bin/ch09_closures_iterateurs.rs)

## Objectifs

Écrire du code expressif en style fonctionnel, avec les performances d'une boucle manuelle.

## Closures

Fonctions anonymes qui **capturent** leur environnement : `|x| x + offset`.

| Trait | Capture | Appelable |
|:--|:--|:--|
| `Fn` | par `&` (lecture) | plusieurs fois, concurremment |
| `FnMut` | par `&mut` | plusieurs fois, en exclusivité |
| `FnOnce` | par valeur (consomme) | **une seule fois** |

`Fn ⊂ FnMut ⊂ FnOnce` : une closure `Fn` peut servir là où `FnMut`/`FnOnce` est attendu. Le mot-clé `move` force la capture par valeur (indispensable pour `thread::spawn`).

Retourner une closure : `impl Fn(i32) -> i32` (statique) ou `Box<dyn Fn(i32) -> i32>` (dynamique).

## Itérateurs

Tout type qui implémente `Iterator` (une seule méthode : `next`) obtient ~75 adaptateurs gratuits.

```rust
let somme: u32 = (1..=100).filter(|n| n % 3 == 0).map(|n| n * n).sum();
```

| Catégorie | Exemples |
|:--|:--|
| Créer | `iter()`, `iter_mut()`, `into_iter()`, `chars()`, `lines()`, `a..b` |
| Adapter (paresseux) | `map`, `filter`, `filter_map`, `flat_map`, `zip`, `enumerate`, `skip`, `take`, `take_while`, `step_by`, `chain`, `rev`, `peekable`, `scan`, `windows`, `chunks` |
| Consommer | `collect`, `sum`, `product`, `count`, `fold`, `min/max(_by_key)`, `any`, `all`, `find`, `position`, `last`, `partition` |

**Paresse** : rien n'est exécuté tant qu'un consommateur n'est pas appelé.
**Coût zéro** : après optimisation, une chaîne d'itérateurs vaut la boucle équivalente (voir chapitre 19).

## À retenir

- Préférez les itérateurs aux boucles à index : plus lisibles, sans vérification de bornes.
- `collect::<Result<Vec<_>, _>>()` transforme « liste de `Result` » en « `Result` de liste ».

## Pièges

- `.map(...)` sans consommateur ne fait rien (le compilateur avertit).
- Capturer une variable par référence puis la déplacer ailleurs : ajoutez `move` ou clonez.

## Exercices

1. Fibonacci en `Iterator` (voir code), puis la somme de ses termes pairs < 4 000 000.
2. Niveau 2, ex. 2.3 (`composer`) et 2.4 (somme des carrés pairs).
3. Récrivez une boucle `for` avec accumulateur en `fold`.

## Pour aller plus loin

[The Book ch. 13](https://doc.rust-lang.org/book/ch13-00-functional-features.html) · [`std::iter`](https://doc.rust-lang.org/std/iter/index.html) · crate `itertools`
