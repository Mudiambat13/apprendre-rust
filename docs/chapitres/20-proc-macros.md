# Chapitre 20 — Macros procédurales

> 🔴 Expert · Code : [`crates/macros_demo/src/lib.rs`](../../crates/macros_demo/src/lib.rs) (à lire d'abord) puis [`src/bin/ch20_proc_macros.rs`](../../src/bin/ch20_proc_macros.rs)

## Principe

Une macro procédurale est une **fonction Rust exécutée à la compilation** : elle reçoit un `TokenStream` et en renvoie un autre. Elle vit dans une crate spéciale (`[lib] proc-macro = true`) et ne peut pas être utilisée dans la crate qui la définit.

## Les trois sortes

| Sorte | Appel | Attribut Rust | Exemple dans ce dépôt |
|:--|:--|:--|:--|
| **derive** | `#[derive(Describe)]` | `#[proc_macro_derive(Describe)]` | ajoute `describe()` et `noms_des_champs()` à une struct |
| **attribut** | `#[chronometre] fn …` | `#[proc_macro_attribute]` | enveloppe une fonction pour mesurer sa durée |
| **fonction** | `carre_const!(12)` | `#[proc_macro]` | calcule `144i64` à la compilation |

## Cette implémentation vs la réalité

Pour rester **sans dépendance**, `macros_demo` analyse les jetons à la main : c'est très instructif mais fragile (pas de génériques, ni d'enums…). En pratique on utilise le trio :

| Crate | Rôle |
|:--|:--|
| **`syn`** | analyse les jetons en un arbre syntaxique typé (`DeriveInput`, `ItemFn`…) |
| **`quote`** | génère des jetons : `quote! { impl #nom { … } }` |
| **`proc-macro2`** | version des `TokenStream` utilisable dans les tests |

Exemple de derive typique :

```rust
#[proc_macro_derive(Describe)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let nom = &ast.ident;
    quote::quote! {
        impl #nom { pub fn nom_du_type() -> &'static str { stringify!(#nom) } }
    }.into()
}
```

## Bonnes pratiques

- **Messages d'erreur** : retournez `compile_error!` / `syn::Error::to_compile_error()` avec le bon `Span`, pas un `panic!` brut.
- **Chemins absolus** dans le code généré (`::std::string::String`), pour éviter les collisions avec les noms de l'utilisateur.
- **Testez** avec `trybuild` (tests de compilation attendus en échec) et `cargo expand`.
- Les procs macros **allongent la compilation** : ne les utilisez que quand une macro déclarative ne suffit pas.

## Exemples réels

`serde` (`Serialize`, `Deserialize`), `clap` (`Parser`), `thiserror` (`Error`), `tokio::main`, `sqlx::query!`, `async-trait`.

## À retenir

- Entrée = jetons, sortie = jetons : **pas de types** à ce stade.
- Un `derive` n'*ajoute* du code qu'à côté de l'élément ; un attribut peut le **remplacer**.

## Exercices

1. Dans `macros_demo`, ajoutez `#[derive(NomsDesChamps)]` qui ne génère que `noms_des_champs()`.
2. Étendez `chronometre` pour accepter un argument : `#[chronometre(etiquette = "calcul")]`.
3. Réécrivez `Describe` avec `syn` + `quote` dans une copie de la crate, et gérez les génériques.

## Pour aller plus loin

[Reference — Procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) · [`syn`](https://docs.rs/syn) · [`quote`](https://docs.rs/quote) · [proc-macro-workshop (dtolnay)](https://github.com/dtolnay/proc-macro-workshop)
