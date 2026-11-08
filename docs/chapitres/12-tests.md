# Chapitre 12 — Tester son code

> 🟡 Intermédiaire · Code : [`src/bin/ch12_tests.rs`](../../src/bin/ch12_tests.rs), [`src/lib.rs`](../../src/lib.rs), [`tests/integration.rs`](../../tests/integration.rs)

## Les trois niveaux (tous intégrés à `cargo test`)

| Niveau | Où | Accès |
|:--|:--|:--|
| **Unitaires** | `#[cfg(test)] mod tests` dans le même fichier | y compris le privé |
| **Intégration** | dossier `tests/` (une crate par fichier) | API publique seulement |
| **Documentation** | blocs de code dans les `///` | compilés et exécutés ! |

## Écrire un test

```rust
#[test]
fn addition() { assert_eq!(additionner(2, 2), 4); }

#[test]
#[should_panic(expected = "division par zéro")]
fn panique() { diviser(1, 0); }

#[test]
fn avec_result() -> Result<(), String> { /* `?` autorisé */ Ok(()) }
```

Assertions : `assert!`, `assert_eq!`, `assert_ne!`, `matches!`, `debug_assert!`. Ajoutez un message : `assert_eq!(a, b, "contexte {x}")`.

**Flottants** : jamais `==` ; comparez avec une tolérance (`(a-b).abs() < 1e-9`).

## Commandes utiles

```bash
cargo test                       # tout
cargo test nom                   # filtrer par nom
cargo test -- --nocapture        # voir les println!
cargo test -- --test-threads=1   # séquentiel
cargo test -- --ignored          # tests marqués #[ignore]
cargo test --doc                 # doctests seulement
```

## Bonnes pratiques

- Un test = un comportement ; un nom qui décrit ce comportement.
- Séparez la logique pure (testable) des effets (I/O) : voir `minigrep::formater`.
- **Tests par propriétés** (`proptest`) : générez des entrées aléatoires et vérifiez des invariants (« décoder(encoder(x)) == x »).
- **Tests de snapshots** (`insta`) pour des sorties volumineuses.
- **Couverture** : `cargo llvm-cov`. **Mutation** : `cargo mutants`.
- **Benchmarks** : `criterion` ou `divan` (le `#[bench]` natif est instable).

## À retenir

- Les doctests garantissent que la documentation ne ment pas.
- Dans `main`, peu de logique : on ne sait pas la tester.

## Exercices

1. Ajoutez des tests pour `est_palindrome` avec des accents.
2. Écrivez un doctest pour une fonction de `src/lib.rs`.
3. Le test `#[ignore]` : lancez-le avec `--ignored --release`.

## Pour aller plus loin

[The Book ch. 11](https://doc.rust-lang.org/book/ch11-00-testing.html) · [Rustdoc — doctests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) · `proptest`, `insta`, `mockall`
