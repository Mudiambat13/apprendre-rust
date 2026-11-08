# Chapitre 1 — Les bases

> 🟢 Débutant · Code : [`src/bin/ch01_bases.rs`](../../src/bin/ch01_bases.rs) · `cargo run --bin ch01_bases`

## Objectifs

- Déclarer des variables (immuables par défaut) et comprendre le *shadowing*.
- Connaître les types scalaires et composés de base.
- Écrire des fonctions et comprendre « expression vs instruction ».

## 1. Cargo, en deux minutes

```bash
cargo new mon_projet     # crée un projet
cargo run                # compile et exécute
cargo build --release    # compile optimisé (mesures de performance : toujours en release)
cargo check              # vérifie sans produire de binaire (rapide)
cargo test               # lance les tests
```

## 2. Variables et mutabilité

```rust
let x = 5;          // immuable
let mut y = 10;     // mutable
const MAX: u32 = 100_000;  // constante : type obligatoire, évaluée à la compilation
```

L'immuabilité par défaut est un choix de conception : le code est plus facile à raisonner, et le compilateur peut mieux optimiser. Le **shadowing** (`let x = …` à nouveau) crée une *nouvelle* variable et permet même de changer de type.

## 3. Types

| Catégorie | Types |
|:--|:--|
| Entiers signés | `i8 i16 i32 i64 i128 isize` |
| Entiers non signés | `u8 u16 u32 u64 u128 usize` |
| Flottants | `f32 f64` |
| Autres scalaires | `bool`, `char` (4 octets, valeur Unicode) |
| Composés | tuple `(i32, f64)`, tableau `[i32; 5]` |

- `isize`/`usize` ont la taille d'un pointeur ; `usize` sert aux indices.
- **Pas de conversion implicite** entre types numériques : `as` (peut tronquer), `From`/`Into` (sans perte), `TryFrom`/`TryInto` (peut échouer).
- Débordement entier : *panic* en debug, *wrapping* en release. Choisissez explicitement : `checked_*`, `wrapping_*`, `saturating_*`, `overflowing_*`.

## 4. Fonctions

```rust
fn carre(n: i32) -> i32 {
    n * n        // pas de `;` : c'est l'expression de retour
}
```

Rust distingue **instructions** (ne produisent pas de valeur) et **expressions** (en produisent une). Un bloc `{ … }`, un `if`, un `match` sont des expressions.

## À retenir

- `let` = immuable, `let mut` = mutable.
- Pas de conversions numériques implicites.
- La dernière expression d'un bloc, sans `;`, est sa valeur.

## Pièges

- Ajouter un `;` à la dernière ligne d'une fonction qui retourne une valeur → erreur « expected i32, found () ».
- `as` tronque silencieusement : `300_i32 as u8 == 44`.

## Exercices

1. Dans `ch01_bases.rs`, retirez `mut` de `y` et lisez le message du compilateur.
2. Écrivez une fonction `celsius_en_fahrenheit(c: f64) -> f64`.
3. Que vaut `i32::MAX.wrapping_add(1)` ? et avec `checked_add` ?
4. Niveau 1 : [`exercices/src/niveau1.rs`](../../exercices/src/niveau1.rs).

## Pour aller plus loin

[The Book ch. 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) · [Rust By Example — Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
