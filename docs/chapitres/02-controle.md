# Chapitre 2 — Flux de contrôle

> 🟢 Débutant · Code : [`src/bin/ch02_controle.rs`](../../src/bin/ch02_controle.rs)

## Objectifs

Maîtriser `if`, les boucles et surtout **`match`**, l'outil le plus important du langage.

## 1. `if` est une expression

```rust
let etat = if temperature > 30 { "chaud" } else { "tempéré" };
```

La condition doit être un `bool` strict (pas de « truthy »). Les deux branches doivent avoir le même type.

## 2. Boucles

| Boucle | Usage |
|:--|:--|
| `loop { … }` | infinie ; `break valeur` renvoie une valeur |
| `while cond { … }` | tant que la condition est vraie |
| `for x in iterable { … }` | **la plus courante** : parcourt un itérateur |

Intervalles : `0..5` (0 à 4) ; `0..=5` (0 à 5). Étiquettes `'externe:` pour sortir de boucles imbriquées.

> Préférez `for x in &v` ou `v.iter().enumerate()` à une boucle sur des index : plus sûr et souvent plus rapide (pas de vérification de bornes).

## 3. `match`

```rust
match n {
    0 => "zéro",
    1..=9 => "un chiffre",
    x if x % 2 == 0 => "pair",   // garde
    _ => "autre",                // cas par défaut
}
```

- **Exhaustif** : oublier un cas est une erreur de compilation. C'est ce qui rend les refactorings sûrs (ajouter une variante à un `enum` signale tous les `match` à mettre à jour).
- Motifs : valeurs, intervalles, `|` (ou), tuples, structs, `@` (liaison), gardes `if`.
- Raccourcis : `if let` (un seul motif), `while let`, `let … else` (sortie anticipée).

## À retenir

- `match` et `if let` sont les outils de base pour consommer `Option`, `Result` et les `enum`.
- Les gardes ne comptent pas dans l'analyse d'exhaustivité : prévoyez un cas général.

## Exercices

1. Écrivez FizzBuzz avec `match (n % 3, n % 5)`. (→ niveau 1, ex. 1.1)
2. Utilisez `while let Some(x) = pile.pop()` pour vider un `Vec`.
3. Réécrivez un `if let … else` en `let … else`.

## Pour aller plus loin

[The Book ch. 3.5-3.6](https://doc.rust-lang.org/book/ch03-05-control-flow.html) · [ch. 6.2 `match`](https://doc.rust-lang.org/book/ch06-02-match.html) · [ch. 18 patterns](https://doc.rust-lang.org/book/ch18-00-patterns.html)
