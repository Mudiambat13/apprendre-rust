# Chapitre 16 — Macros déclaratives

> 🟠 Avancé · Code : [`src/bin/ch16_macros.rs`](../../src/bin/ch16_macros.rs)

## Principe

Une macro **génère du code avant la vérification des types**. `macro_rules!` fonctionne par **filtrage de motifs** sur des jetons.

```rust
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($reste:expr),+) => {{
        let (a, b) = ($x, max!($($reste),+));
        if a > b { a } else { b }
    }};
}
```

## Fragments

`expr`, `ident`, `ty`, `pat`, `stmt`, `block`, `item`, `path`, `literal`, `lifetime`, `vis`, `tt` (arbre de jetons : le « joker »).

## Répétitions

| Syntaxe | Sens |
|:--|:--|
| `$( … ),*` | zéro ou plus, séparés par `,` |
| `$( … );+` | un ou plus, séparés par `;` |
| `$( … )?` | optionnel (ex. virgule finale `$(,)?`) |

## Hygiène

Les variables locales déclarées *dans* la macro n'entrent pas en collision avec celles de l'appelant (voir `hygiene!` dans le code). En revanche, les **identifiants passés en argument** sont ceux de l'appelant.

## Quand utiliser une macro ?

| ✔ Bon usage | ✘ Mauvais usage |
|:--|:--|
| nombre variable d'arguments (`vec!`, `println!`) | ce qu'une fonction ou un générique sait faire |
| réduire du code répétitif (`impl Trait for A, B, C`) | masquer de la logique complexe |
| DSL, génération de tests paramétrés | quand le message d'erreur deviendrait illisible |

## Déboguer

- `cargo expand` montre le code généré (`cargo install cargo-expand`).
- `trace_macros!` (nightly) et `compile_error!("message")` pour signaler un usage invalide.
- Évaluez chaque argument **une seule fois** (`let a = $x;`) pour éviter des effets de bord doublés.

## Macros standard à connaître

`println!`, `format!`, `vec!`, `assert!`/`assert_eq!`, `matches!`, `dbg!`, `todo!`, `unimplemented!`, `unreachable!`, `stringify!`, `concat!`, `include_str!`, `env!`, `cfg!`, `write!`.

## À retenir

- Les macros déclaratives sont puissantes mais difficiles à lire : commentez-les.
- Pour dériver des traits ou analyser du code structuré → macros **procédurales** (chapitre 20).

## Exercices

1. Écrivez `ensemble![1, 2, 3]` qui construit un `HashSet`.
2. Une macro `impl_from_enum!` qui implémente `From<T>` pour chaque variante d'un enum.
3. Une macro `tests_carres!` qui génère un `#[test]` par paire (voir le code) ; ajoutez les cubes.

## Pour aller plus loin

[The Book ch. 19.5](https://doc.rust-lang.org/book/ch19-06-macros.html) · [*The Little Book of Rust Macros*](https://veykril.github.io/tlborm/) · [Reference — Macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)
