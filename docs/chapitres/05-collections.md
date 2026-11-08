# Chapitre 5 — Collections

> 🟢 Débutant · Code : [`src/bin/ch05_collections.rs`](../../src/bin/ch05_collections.rs)

## Objectifs

Choisir et utiliser `Vec`, `String`, `HashMap` et leurs cousins.

## Choisir une collection

| Besoin | Type | Coût typique |
|:--|:--|:--|
| Suite ordonnée, accès par indice | `Vec<T>` | accès O(1), ajout en fin O(1) amorti |
| Ajout/retrait aux **deux** extrémités | `VecDeque<T>` | O(1) |
| Associer clé → valeur | `HashMap<K, V>` | O(1) moyen, ordre aléatoire |
| Clé → valeur **triée**, intervalles | `BTreeMap<K, V>` | O(log n) |
| Ensemble sans doublons | `HashSet<T>` / `BTreeSet<T>` | idem |
| Plus grand élément en priorité | `BinaryHeap<T>` | push/pop O(log n) |

Par défaut : `Vec`, et `HashMap` pour les dictionnaires. Sortez le reste quand un besoin précis l'exige.

## `Vec<T>`

- `vec![1, 2, 3]`, `push`, `pop`, `insert`, `remove`, `retain`, `sort`, `dedup`, `extend`.
- `v[i]` panique hors limites ; `v.get(i)` renvoie une `Option`.
- `Vec::with_capacity(n)` évite les réallocations quand on connaît la taille.

## `String` et UTF-8

- `String` = buffer UTF-8 possédé ; `&str` = vue. Pas d'indexation par entier (`s[0]` n'existe pas) : un caractère peut faire 1 à 4 octets.
- Parcourir : `s.chars()` (caractères), `s.bytes()`, `s.char_indices()`.
- Concaténer : `+` (déplace la gauche), `format!`, `push_str`.

## `HashMap`

```rust
*compteur.entry(mot).or_insert(0) += 1;   // l'API `entry` : LE pattern à connaître
```

L'ordre d'itération est volontairement **aléatoire** (protection contre les attaques par collisions). Besoin d'un ordre → `BTreeMap` ou trier les clés.

## À retenir

- `.collect()` construit presque n'importe quelle collection depuis un itérateur (chapitre 9).
- Les collections **possèdent** leurs éléments : insérer déplace la valeur.

## Pièges

- Garder une référence vers un élément puis `push` : refusé par le borrow checker (la réallocation invaliderait la référence).
- `dedup` ne retire que les doublons **consécutifs** : triez d'abord.

## Exercices

1. Fréquence des mots d'un texte (niveau 1, ex. 1.5).
2. Fusionnez deux `HashMap` en additionnant les valeurs des clés communes.
3. Écrivez un « anagramme » : regroupez des mots ayant les mêmes lettres (`HashMap<Vec<char>, Vec<&str>>`).

## Pour aller plus loin

[The Book ch. 8](https://doc.rust-lang.org/book/ch08-00-common-collections.html) · [`std::collections` — quelle collection choisir](https://doc.rust-lang.org/std/collections/index.html)
