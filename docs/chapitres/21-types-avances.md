# Chapitre 21 — Système de types avancé

> 🔴 Expert · Code : [`src/bin/ch21_types_avances.rs`](../../src/bin/ch21_types_avances.rs)

## 1. Génériques constants (`const generics`)

Des **valeurs** comme paramètres de type : `Matrice<const L: usize, const C: usize>`. Le compilateur vérifie les dimensions : multiplier une 2×3 par une 2×3 **ne compile pas**. Les tableaux `[T; N]` en sont l'exemple natif.

## 2. `const fn` : calcul à la compilation

Une `const fn` peut être appelée dans un contexte constant (`const X: u64 = factorielle(10);`, tailles de tableau, `static`). Les restrictions diminuent à chaque version. `const _: () = assert!(…);` vérifie un invariant **à la compilation**.

## 3. HRTB — *Higher-Ranked Trait Bounds*

`F: for<'a> Fn(&'a str) -> &'a str` : « pour **tout** lifetime `'a` ». Nécessaire quand la fonction appelle la closure avec des références qu'elle crée **elle-même** (donc de lifetime inconnu de l'appelant).

## 4. `Send` et `Sync` (traits « auto »)

| Type | `Send` | `Sync` |
|:--|:-:|:-:|
| `i32`, `String`, `Vec<T>` | ✔ | ✔ |
| `Rc<T>` | ✘ | ✘ |
| `RefCell<T>` / `Cell<T>` | ✔ | ✘ |
| `Arc<T>` | ✔ si `T: Send+Sync` | ✔ si `T: Send+Sync` |
| `Mutex<T>` | ✔ si `T: Send` | ✔ si `T: Send` |
| pointeurs bruts | ✘ | ✘ |

## 5. Niches et disposition

`Option<&T>`, `Option<Box<T>>`, `Option<NonZeroU32>`, `Option<bool>` ont la **même taille** que leur contenu : la valeur « interdite » (zéro, valeurs inutilisées d'un `bool`) encode `None`.

## 6. `Pin` et types auto-référentiels

Un type qui contient un pointeur vers lui-même ne doit plus bouger. `Pin<P>` empêche le déplacement ; `PhantomPinned` (type `!Unpin`) l'impose. C'est la base du fonctionnement des futurs `async` (chapitre 14).

## 7. Variance et `PhantomData`

- **Covariant** : `&'long T` est utilisable là où `&'short T` est attendu.
- **Invariant** : `&mut T`, `Cell<T>` (pas de sous-typage possible).
- `PhantomData<T>` indique au compilateur « je me comporte comme si je possédais/référençais un `T` » (variance, `Send`/`Sync`, *drop check*). Détails : Rustonomicon.

## 8. Ordre de destruction

Variables locales : **ordre inverse** de déclaration. Champs d'une struct : **ordre de déclaration**. `let _ = valeur;` détruit **immédiatement** (mais `let _x = valeur;` conserve jusqu'à la fin du bloc). En édition 2024, les temporaires en fin de bloc sont détruits **avant** les variables locales.

## À retenir

- Plus vous encodez de contraintes **dans les types**, moins il y a d'erreurs à l'exécution.
- Ces outils servent surtout à écrire des **bibliothèques** ; une application s'en passe souvent.

## Exercices

1. Niveau 4, ex. 4.2 : `TamponCirculaire<T, const N: usize>`.
2. Écrivez `fn premier<const N: usize>(t: [i32; N]) -> Option<i32>` et expliquez pourquoi `[i32; 0]` vaut `None`.
3. Pourquoi `Rc<i32>` n'est-il pas `Send` ? Que casserait-il ?

## Pour aller plus loin

[Reference — Const generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics) · [Nomicon — Subtyping and variance](https://doc.rust-lang.org/nomicon/subtyping.html) · [`std::pin`](https://doc.rust-lang.org/std/pin/index.html) · *Rust for Rustaceans* ch. 1 et 3
