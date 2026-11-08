# Chapitre 4 — Structs, enums et `Option`

> 🟢 Débutant · Code : [`src/bin/ch04_structs_enums.rs`](../../src/bin/ch04_structs_enums.rs)

## Objectifs

Modéliser des données avec des **types produit** (`struct`) et des **types somme** (`enum`), et faire du « make illegal states unrepresentable ».

## 1. Structs

```rust
#[derive(Debug, Clone, Default)]
struct Utilisateur { nom: String, actif: bool }

impl Utilisateur {
    fn new(nom: &str) -> Self { Self { nom: nom.into(), actif: true } }  // fonction associée
    fn desactiver(&mut self) { self.actif = false; }                      // méthode
}
```

- Variantes : struct-tuple (`struct Metres(f64)` — *newtype*), struct unité.
- `#[derive(…)]` génère `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`, `Default`.
- Syntaxe de mise à jour : `Utilisateur { nom: "x".into(), ..autre }`.

## 2. Enums : des variantes qui portent des données

```rust
enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
}
```

Là où d'autres langages utilisent des booléens et des champs « parfois valides », Rust encode l'état dans le type : on ne peut lire `x` que dans la variante `Deplacer`, et `match` oblige à traiter toutes les variantes.

## 3. `Option<T>` : pas de `null`

```rust
enum Option<T> { None, Some(T) }
```

Le type force à traiter l'absence. Combinateurs utiles : `map`, `and_then`, `unwrap_or`, `unwrap_or_default`, `ok_or`, `filter`, `take`, `?`.

## 4. Quand struct, quand enum ?

| Situation | Choix |
|:--|:--|
| Plusieurs champs toujours présents ensemble | `struct` |
| « Un parmi N possibilités » | `enum` |
| Des champs facultatifs dépendant d'un état | `enum` (sinon `Option` partout = états invalides possibles) |

## À retenir

- Un `enum` est le moyen idiomatique d'exprimer une machine à états ou un protocole.
- `#[derive(Debug)]` partout en développement : `println!("{x:?}")` et `{x:#?}`.

## Exercices

1. Modélisez une `Forme` (cercle, rectangle, triangle) en enum avec une méthode `aire`.
2. Ajoutez une variante à `Message` et regardez le compilateur lister les `match` à compléter.
3. Niveau 1 : `Rectangle` (ex. 1.6).

## Pour aller plus loin

[The Book ch. 5-6](https://doc.rust-lang.org/book/ch05-00-structs.html) · [Rust By Example — Custom types](https://doc.rust-lang.org/rust-by-example/custom_types.html)
