# Chapitre 6 — Gestion des erreurs

> 🟢 Débutant · Code : [`src/bin/ch06_erreurs.rs`](../../src/bin/ch06_erreurs.rs)

## Objectifs

Distinguer erreurs récupérables et irrécupérables, propager proprement avec `?`, définir ses propres types d'erreur.

## 1. Deux familles

| | Mécanisme | Quand |
|:--|:--|:--|
| Bug / invariant violé | `panic!`, `unwrap`, `expect`, `assert!` | l'erreur n'a pas de sens à gérer |
| Erreur attendue (entrée, I/O, réseau) | `Result<T, E>` | l'appelant doit décider |

Pas d'exceptions : l'erreur **fait partie de la signature**, et ne peut pas être ignorée par inadvertance (`#[must_use]`).

## 2. L'opérateur `?`

```rust
fn lire(chemin: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(chemin)?;      // si Err, retourne immédiatement l'erreur
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

`?` appelle `From::from` sur l'erreur : c'est ce qui permet de convertir automatiquement des erreurs de types différents vers l'erreur de votre fonction.

## 3. Définir ses erreurs

- Implémenter `Display` + `std::error::Error` (et `source()` pour chaîner les causes).
- Dans une **bibliothèque** : un `enum` précis, que l'appelant peut `match`er (la crate `thiserror` supprime le code répétitif).
- Dans une **application** : `Box<dyn Error>` ou la crate `anyhow` (avec du contexte : `.context("lecture de la config")`).

## 4. `unwrap` : quand est-ce acceptable ?

Tests, prototypes, exemples, ou quand l'échec est **réellement impossible** — dans ce cas préférez `expect("raison de l'invariant")`, qui documente pourquoi.

## 5. Combinateurs

`map`, `map_err`, `and_then`, `ok_or`, `unwrap_or`, `unwrap_or_else`, `ok()`. Un itérateur de `Result` se `collect()` en `Result<Vec<_>, _>` et s'arrête à la première erreur.

## À retenir

- `Option` = « il peut ne rien y avoir » ; `Result` = « ça peut échouer, et voici pourquoi ».
- Gardez la **chaîne de causes** (`source()`) : elle rend les logs exploitables.

## Pièges

- `unwrap()` dans du code de production sur des entrées utilisateur.
- Convertir toutes les erreurs en `String` : on perd la capacité de les traiter selon leur nature.

## Exercices

1. Écrivez un parseur de configuration `clé=valeur` qui retourne des erreurs typées (voir le code).
2. Niveau 1, ex. 1.4 : somme d'une liste CSV avec `Result`.
3. Faites réussir `main() -> Result<(), Box<dyn Error>>` avec un fichier absent : observez le code de sortie.

## Pour aller plus loin

[The Book ch. 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html) · crates [`thiserror`](https://docs.rs/thiserror) et [`anyhow`](https://docs.rs/anyhow) · article *Error Handling in Rust* (Andrew Gallant)
