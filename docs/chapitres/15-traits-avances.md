# Chapitre 15 — Traits avancés

> 🟠 Avancé · Code : [`src/bin/ch15_traits_avances.rs`](../../src/bin/ch15_traits_avances.rs)

## Ce que couvre le chapitre

| Notion | Idée |
|:--|:--|
| **Type associé** (`type Element;`) | un type « choisi par l'implémenteur » : *une* implémentation par type, contrairement à un paramètre générique (`Trait<T>`) qui permet plusieurs |
| **Constante associée** | `const NOM: &str;` dans un trait |
| **GAT** (types associés génériques) | `type Item<'a>` : permet les itérateurs qui *prêtent* leurs éléments (impossibles avec `Iterator`) |
| **Surcharge d'opérateurs** | traits de `std::ops` : `Add`, `Sub`, `Mul`, `Neg`, `Index`, `AddAssign`… |
| **Supertrait** | `trait Animal: Display` — exige un autre trait |
| **Trait d'extension** | ajouter des méthodes à un type externe via un nouveau trait |
| **Typestate** | encoder l'état d'un objet dans son type (`Porte<Ouverte>`) avec `PhantomData` : les transitions invalides **ne compilent pas** |
| **`dyn Any`** | typage dynamique et *downcast* (à utiliser avec parcimonie) |

## Type associé ou paramètre générique ?

- *Un type donné n'a qu'une seule relation naturelle* (ex. `Iterator::Item`) → type associé.
- *Plusieurs relations possibles* (ex. `From<A>` et `From<B>` pour le même type) → paramètre générique.

## Typestate : le compilateur comme vérificateur de protocole

```rust
let porte = Porte::<Fermee>::new("entrée");
let porte = porte.ouvrir();         // Porte<Ouverte>
// porte.verrouiller();             // ne compile pas : impossible depuis « ouverte »
```

C'est un pattern très utilisé pour les builders, connexions, états d'API (ex. `hyper`, HAL embarqués). Coût d'exécution : **zéro** (`PhantomData` occupe 0 octet).

## Opérateurs : règles de bonne conduite

- Respectez la sémantique mathématique attendue ; sinon préférez une méthode nommée.
- `impl Add for T` consomme les opérandes ; pour des types lourds, implémentez aussi `impl Add<&T> for &T`.

## À retenir

- Les traits sont l'outil de conception central : apprenez à reconnaître *quand* un type associé est la bonne abstraction.
- Le typestate transforme des erreurs d'exécution en erreurs de compilation.

## Exercices

1. Niveau 3, ex. 3.2 : `Vecteur2` avec `Add`, `Sub`, `Mul<f64>`, `Neg`.
2. Ajoutez `Mul<Vec2> for f64` pour pouvoir écrire `2.0 * v`.
3. Écrivez un typestate pour une connexion : `Deconnectee → Connectee → Authentifiee`.

## Pour aller plus loin

[The Book ch. 19](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html) · *Rust for Rustaceans* ch. 3 · [Reference — Associated items](https://doc.rust-lang.org/reference/items/associated-items.html) · [`std::ops`](https://doc.rust-lang.org/std/ops/index.html)
