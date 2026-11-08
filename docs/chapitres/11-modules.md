# Chapitre 11 — Modules, crates et workspaces

> 🟡 Intermédiaire · Code : [`src/bin/ch11_modules.rs`](../../src/bin/ch11_modules.rs) · projet [`minigrep`](../../projets/minigrep)

## Vocabulaire

| Terme | Définition |
|:--|:--|
| **Crate** | unité de compilation : binaire (`main.rs`) ou bibliothèque (`lib.rs`) |
| **Package** | un `Cargo.toml` + ses crates (au plus une lib, plusieurs binaires) |
| **Module** | espace de noms dans une crate (`mod`) |
| **Workspace** | plusieurs packages partageant `Cargo.lock` et `target/` |

## Visibilité

Tout est **privé par défaut**. `pub` expose ; `pub(crate)` limite à la crate ; `pub(super)` au parent. Un champ de struct est privé sauf `pub` : on garde ainsi les invariants (constructeur obligatoire).

## Modules et fichiers

```
src/lib.rs                 // mod reseau;
src/reseau.rs              // pub mod tcp;
src/reseau/tcp.rs
```

Chemins : `crate::a::b` (absolu), `self::b`, `super::b`. `use` importe ; `use … as …` renomme ; `pub use` **ré-exporte** (façade d'API).

## Bonne architecture

- **Séparez `lib.rs` et `main.rs`** : la logique dans la bibliothèque (testable), `main` mince (arguments, affichage, code de sortie). Voir `minigrep`.
- Peu d'API publiques, bien documentées (`///`), exemples compilés comme tests.
- Un module `prelude` regroupe les imports usuels de votre crate.

## Cargo avancé

- **Features** : fonctionnalités optionnelles (`[features]`, `cfg(feature = "…")`).
- **Profils** : `[profile.release]` (`lto`, `codegen-units`, `panic = "abort"`).
- **Dépendances** : `cargo add serde`, versions SemVer (`^1.2`), `cargo update`, `cargo tree`.
- **Publication** : `cargo publish` (crates.io), `cargo doc`, `cargo semver-checks`.

## À retenir

- Exposez le minimum. Retirer un `pub` plus tard casse vos utilisateurs ; en ajouter est sans risque.
- Le `Cargo.toml` racine de ce dépôt est un exemple de workspace complet.

## Exercices

1. Passez `ch11_modules.rs` en arborescence de fichiers.
2. Ajoutez une option `-w` (mot entier) à `minigrep` + des tests.
3. Créez un workspace avec une lib `geometrie` et un binaire qui l'utilise.

## Pour aller plus loin

[The Book ch. 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) · [Cargo Book](https://doc.rust-lang.org/cargo/) · [API Guidelines](https://rust-lang.github.io/api-guidelines/)
