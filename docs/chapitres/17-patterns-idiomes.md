# Chapitre 17 — Patterns et idiomes

> 🟠 Avancé · Code : [`src/bin/ch17_patterns_idiomes.rs`](../../src/bin/ch17_patterns_idiomes.rs)

Écrire du Rust qui *compile* est une chose ; écrire du Rust **idiomatique** en est une autre. Voici le répertoire commun.

## Patterns

| Pattern | Problème résolu | Où le voir |
|:--|:--|:--|
| **Newtype** | éviter de mélanger deux `f64` (euros/dollars) ; implémenter un trait externe | `Euros(f64)` |
| **Builder** | objets à nombreux paramètres facultatifs, validation à la fin | `RequeteBuilder` |
| **RAII / Guard** | garantir un nettoyage même en cas d'erreur ou de panic | `Garde`, `MutexGuard`, `File` |
| **Typestate** | états invalides non représentables | chapitre 15 |
| **Machine à états par `enum`** | transitions explicites | `Commande` |
| **Strategy** | comportement interchangeable | `Box<dyn Fn>` / traits |
| **Visitor / `match`** | opérer sur un AST | calculatrice |
| **Extension trait** | ajouter des méthodes à `str`, `Iterator`… | chapitre 15 |

## Idiomes à adopter

- **`From` / `Into`** pour les conversions ; `TryFrom` quand elles peuvent échouer ; `AsRef<str>` pour accepter `&str` *et* `String`.
- **`Default`** + syntaxe `..Default::default()` pour les configurations.
- **`Cow<'a, str>`** : n'allouer que si on modifie réellement.
- **`mem::take` / `replace` / `swap`** pour déplacer un champ depuis un `&mut self`.
- **Itérateurs plutôt que boucles** à index ; `?` plutôt que `match` sur `Result`.
- **Constructeur `new`** + `Default` ; méthodes de conversion `as_` (gratuite, emprunt), `to_` (coûteuse, nouvelle valeur), `into_` (consomme).
- **Accepter des types généraux, retourner des types précis** : `fn f(x: &str)` mais `-> String`.
- **Éviter `clone()` réflexe** et `unwrap()` hors tests.
- **`#[must_use]`** sur les fonctions dont le résultat ne doit pas être ignoré.
- **`#[non_exhaustive]`** sur les `enum`/`struct` publics susceptibles d'évoluer.
- Nommage : `snake_case` (fonctions, variables, modules), `CamelCase` (types, traits), `SCREAMING_SNAKE_CASE` (constantes).

## Anti-patterns

| Anti-pattern | Alternative |
|:--|:--|
| `Rc<RefCell<…>>` partout pour « faire taire » le compilateur | repenser la propriété, utiliser des indices ou un arène |
| Héritage simulé avec `Deref` | composition + traits |
| `unwrap()` en production | `?`, `expect` avec raison, gestion explicite |
| `String` pour tout | types dédiés (newtypes, enums) |
| Un trait « fourre-tout » énorme | petits traits cohérents |
| Booléens multiples d'état | un `enum` |

## À retenir

- Le but n'est pas la ruse : un code Rust idiomatique est **évident** à lire.
- Lancez `cargo clippy` : il enseigne les idiomes en continu.

## Exercices

1. Transformez une struct à 6 paramètres de constructeur en builder.
2. Écrivez un newtype `Email` dont le constructeur valide le format (`TryFrom<&str>`).
3. Implémentez un garde RAII qui mesure et affiche la durée d'un bloc.

## Pour aller plus loin

[Rust Design Patterns](https://rust-unofficial.github.io/patterns/) · [API Guidelines](https://rust-lang.github.io/api-guidelines/) · [*Effective Rust*](https://www.lurklurk.org/effective-rust/)
