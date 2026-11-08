# Chapitre 8 — Durées de vie (*lifetimes*)

> 🟡 Intermédiaire · Code : [`src/bin/ch08_lifetimes.rs`](../../src/bin/ch08_lifetimes.rs)

## Objectifs

Comprendre ce que dit une annotation `'a` — et pourquoi, le plus souvent, vous n'avez pas à l'écrire.

## L'idée

Une référence ne doit jamais survivre à la donnée qu'elle désigne. Le compilateur vérifie cela avec des **régions** nommées *lifetimes*. Une annotation `'a` **ne prolonge rien** : elle *décrit* une relation entre des références que le compilateur vérifie.

```rust
fn plus_long<'a>(x: &'a str, y: &'a str) -> &'a str { … }
```

Lecture : « le résultat est valable au plus aussi longtemps que **les deux** arguments ».

## Les règles d'élision

Dans la plupart des cas, le compilateur déduit les lifetimes :

1. Chaque référence en entrée reçoit son propre lifetime.
2. S'il n'y a **qu'un** lifetime en entrée, il est attribué à toutes les sorties.
3. S'il y a `&self` / `&mut self`, son lifetime est attribué aux sorties.

Annotation obligatoire quand ces règles ne suffisent pas (ex. deux entrées, une sortie, pas de `self`).

## Structs qui contiennent des références

```rust
struct Extrait<'a> { partie: &'a str }
```

L'`Extrait` ne peut pas survivre au texte dont il vient. Alternative souvent plus simple : stocker des données **possédées** (`String`) ; réservez les références aux structures temporaires (parseurs, itérateurs, vues).

## `'static`

- `&'static str` : donnée valable pour tout le programme (littéraux).
- `T: 'static` signifie « ne contient aucune référence à durée limitée » (un `String` est `'static`), **pas** « vit éternellement ».

## Techniques de survie

| Problème | Solution |
|:--|:--|
| « Le lifetime devient ingérable » | stocker des valeurs possédées, ou `Rc`/`Arc` |
| Retourner une référence vers une variable locale | retourner la valeur (`String`) |
| Structure auto-référentielle | repenser (indices), ou `Pin` (chapitre 21) |
| Closure qui accepte n'importe quelle référence | HRTB `for<'a>` (chapitre 21) |

## À retenir

- Les lifetimes sont de la **documentation vérifiée** de qui emprunte quoi.
- Un message « borrowed value does not live long enough » = votre référence vit plus longtemps que sa source.

## Exercices

1. Dans `ch08_lifetimes.rs`, utilisez `resultat` hors du bloc et lisez l'erreur.
2. Écrivez `fn mot_le_plus_long<'a>(phrase: &'a str) -> &'a str`.
3. Niveau 2, ex. 2.1.

## Pour aller plus loin

[The Book ch. 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) · [Nomicon — Lifetimes](https://doc.rust-lang.org/nomicon/lifetimes.html) · *Common Rust Lifetime Misconceptions* (pretzelhammer)
