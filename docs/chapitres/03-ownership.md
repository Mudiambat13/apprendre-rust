# Chapitre 3 — Ownership, emprunts et slices

> 🟢 Débutant · Code : [`src/bin/ch03_ownership.rs`](../../src/bin/ch03_ownership.rs)
> ⭐ **Le chapitre le plus important.** Relisez-le après chaque niveau.

## Objectifs

Comprendre comment Rust garantit la sécurité mémoire **sans ramasse-miettes** et **sans erreurs de gestion manuelle**.

## 1. Pile et tas (rappel)

- **Pile** : rapide, taille connue à la compilation (`i32`, `bool`, tableaux fixes…).
- **Tas** : taille dynamique (`String`, `Vec`, `Box`) ; l'allocation coûte plus cher. Une valeur sur le tas est désignée par un pointeur stocké sur la pile.

## 2. Les trois règles

1. Chaque valeur a **un unique propriétaire**.
2. Quand le propriétaire sort de portée, la valeur est **détruite** (`Drop`) — c'est le principe **RAII**.
3. Une valeur peut être **déplacée** (*move*) vers un autre propriétaire ; l'ancien n'est alors plus utilisable.

```rust
let a = String::from("salut");
let b = a;            // move : `a` est invalidée
// println!("{a}");   // ERREUR
let c = b.clone();    // copie profonde, explicite
```

Les types `Copy` (entiers, `bool`, `char`, `&T`, tuples de `Copy`…) sont copiés au lieu d'être déplacés.

## 3. Emprunts (références)

| Forme | Droit | Combien à la fois |
|:--|:--|:--|
| `&T` | lecture | autant qu'on veut |
| `&mut T` | lecture + écriture | **un seul**, et aucun `&T` en même temps |

Cette règle (« partage XOR mutation ») empêche à la compilation : les *data races*, l'invalidation d'itérateurs, les pointeurs pendants (*use-after-free*).

**NLL** (*Non-Lexical Lifetimes*) : un emprunt dure jusqu'à sa **dernière utilisation**, pas jusqu'à la fin du bloc.

## 4. Slices

`&str` (vue sur du texte), `&[T]` (vue sur une suite). Une slice = pointeur + longueur. **Règle d'API** : acceptez `&str` plutôt que `&String`, `&[T]` plutôt que `&Vec<T>`.

⚠ Les indices d'un `&str` sont des **octets** UTF-8, pas des caractères : `"été".len() == 5`.

## 5. Modèle mental

> Un propriétaire = la clé d'un appartement. Vous pouvez **donner** la clé (move), **prêter** une copie en lecture à plusieurs personnes (`&T`), ou prêter l'unique clé de travaux à une seule personne (`&mut T`).

## À retenir

- Si le compilateur râle, il vous protège d'un vrai bug. Lisez le message en entier : il propose souvent la correction.
- En cas de doute : empruntez (`&`) plutôt que de déplacer ; clonez en dernier recours, pas en premier réflexe.

## Pièges courants

- Cloner partout « pour que ça compile » : cela marche, mais masque une conception à revoir.
- Retourner une référence vers une variable locale → voir le chapitre 8.
- Modifier une collection pendant qu'on l'itère.

## Exercices

1. Provoquez chaque erreur commentée dans `ch03_ownership.rs` (décommentez-les une à une).
2. Écrivez `fn plus_long_mot(texte: &str) -> &str`.
3. Pourquoi `let s: str = "abc";` ne compile-t-il pas ? (indice : types de taille dynamique)

## Pour aller plus loin

[The Book ch. 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) · [Version interactive avec quiz (Brown)](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html) · Rustlings `move_semantics`
