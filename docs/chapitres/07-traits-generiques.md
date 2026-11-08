# Chapitre 7 — Génériques et traits

> 🟡 Intermédiaire · Code : [`src/bin/ch07_traits_generiques.rs`](../../src/bin/ch07_traits_generiques.rs)

## Objectifs

Écrire du code réutilisable sans perdre en performance, et définir des comportements partagés.

## 1. Génériques et monomorphisation

```rust
fn plus_grand<T: PartialOrd>(liste: &[T]) -> Option<&T> { … }
```

Le compilateur génère une version spécialisée par type utilisé (**monomorphisation**) : aucune surcharge à l'exécution, au prix d'un binaire un peu plus gros.

Contraintes : `T: A + B`, ou `where T: A, U: B` pour la lisibilité.

## 2. Traits

Un trait décrit un comportement : méthodes requises + méthodes par défaut.

```rust
trait Resumable {
    fn auteur(&self) -> String;
    fn resume(&self) -> String { format!("(plus de {}…)", self.auteur()) }
}
```

Traits standard à connaître : `Debug`, `Display`, `Clone`, `Copy`, `PartialEq`/`Eq`, `PartialOrd`/`Ord`, `Hash`, `Default`, `From`/`Into`, `Iterator`, `Drop`, `Deref`, `AsRef`, `Send`/`Sync`.

**Règle de l'orphelin** : vous pouvez implémenter un trait sur un type seulement si le trait *ou* le type vous appartient. Contournement : le *newtype* (chapitre 17).

## 3. `impl Trait`, génériques et `dyn Trait`

| Forme | Dispatch | Usage |
|:--|:--|:--|
| `fn f(x: impl Trait)` / `fn f<T: Trait>(x: T)` | statique (monomorphisé) | par défaut |
| `fn f() -> impl Trait` | statique, type masqué | retourner une closure/itérateur |
| `Box<dyn Trait>` / `&dyn Trait` | **dynamique** (vtable) | collections hétérogènes, plugins |

`dyn Trait` exige que le trait soit *dyn-compatible* (pas de méthode générique, pas de `Self` en retour…).

## 4. Implémentations générales (*blanket impl*)

```rust
impl<T: Display> ToString for T { … }   // c'est ainsi que std fournit `to_string()` partout
```

## À retenir

- Génériques/`impl Trait` pour la performance ; `dyn Trait` pour la flexibilité d'exécution.
- Un trait = un contrat ; privilégiez de petits traits cohérents.

## Pièges

- Oublier une contrainte (`T: Display`) : l'erreur indique exactement laquelle manque.
- Tenter de mettre `Vec<impl Trait>` avec des types différents : utilisez `Vec<Box<dyn Trait>>`.

## Exercices

1. Implémentez `Display` pour une struct `Fraction`.
2. Écrivez un trait `Forme` + `aire_totale(&[Box<dyn Forme>])` (niveau 2, ex. 2.5).
3. Pile générique `Pile<T>` (niveau 2, ex. 2.2).

## Pour aller plus loin

[The Book ch. 10](https://doc.rust-lang.org/book/ch10-00-generics.html) · [ch. 18 « objets-traits »](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) · *Rust for Rustaceans* ch. 2
