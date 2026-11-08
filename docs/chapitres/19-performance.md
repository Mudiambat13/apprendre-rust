# Chapitre 19 — Performance

> 🔴 Expert · Code : [`src/bin/ch19_performance.rs`](../../src/bin/ch19_performance.rs) · `cargo run --release --bin ch19_performance`

## Règles d'or

1. **Mesurez, ne devinez pas.** Profilez avant d'optimiser.
2. **Toujours `--release`** pour toute mesure (le debug est 10 à 100× plus lent).
3. Optimisez d'abord l'**algorithme et les structures de données**, puis la mémoire, en dernier les micro-détails.
4. Un code clair et correct d'abord ; le rapide ensuite, là où le profil le justifie.

## Leviers, du plus rentable au moins rentable

| Levier | Détail |
|:--|:--|
| **Algorithme / structure** | O(n²) → O(n log n) bat toute micro-optimisation |
| **Allocations** | `Vec::with_capacity`, réutiliser les buffers, `write!` plutôt que `format!` en boucle, `SmallVec`/`ArrayVec` |
| **Localité du cache** | parcours contigus et séquentiels ; *struct of arrays* ; éviter `Vec<Box<T>>` |
| **Itérateurs** | coût zéro, vectorisables (SIMD automatique) |
| **Dispatch** | génériques (inlinés) vs `dyn` (vtable, pas d'inlining) |
| **Clones inutiles** | emprunter, `Cow`, `Rc`/`Arc` |
| **Parallélisme** | `rayon` pour les données ; threads pour les tâches |
| **Profil de compilation** | `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, `opt-level = 3` ; `target-cpu=native` (non portable) |

## Mesurer

| Outil | Rôle |
|:--|:--|
| `std::time::Instant` + `black_box` | mesure rapide (ce chapitre) |
| **criterion** / **divan** | benchmarks statistiques fiables |
| **cargo flamegraph** / `perf` | où passe le temps CPU |
| **heaptrack** / **dhat** | où sont les allocations |
| **cachegrind** / `perf stat` | défauts de cache, branchements |
| **Godbolt** | lire l'assembleur généré |
| `cargo bloat`, `cargo llvm-lines` | taille du binaire, code dupliqué |

## Disposition mémoire

`size_of`, `align_of`. Le compilateur **réordonne** les champs d'une struct (`repr(Rust)`) pour minimiser le *padding* ; `#[repr(C)]` impose l'ordre. Les **niches** (références, `Box`, `NonZero*`, `bool`) rendent `Option<T>` gratuit.

## Ce que le programme du chapitre montre

- `with_capacity` contre réallocations successives.
- Boucle `for` contre chaîne d'itérateurs.
- Parcours d'une matrice **par lignes** (rapide) vs **par colonnes** (lent, défauts de cache) : même quantité de calcul.
- Appel générique contre appel via `dyn` : souvent **identiques** quand le compilateur peut dévirtualiser ; l'écart apparaît avec des types vraiment hétérogènes (exercice : essayez un `Vec<Box<dyn Operation>>`).
- Tailles mémoire de types courants.

> Les durées affichées dépendent de votre machine : comparez les **rapports**, pas les valeurs absolues.

## À retenir

- Les abstractions de Rust sont à **coût zéro** : l'idiomatique est souvent aussi le plus rapide.
- Le « Rust lent » vient presque toujours d'un `debug`, de clones/allocations en boucle, ou d'un mauvais algorithme.

## Exercices

1. Profilez un de vos programmes avec `cargo flamegraph`.
2. Remplacez `HashMap<String, _>` par `HashMap<&str, _>` dans une boucle chaude et mesurez.
3. Écrivez un benchmark `criterion` comparant `Vec` et `VecDeque` pour des insertions en tête.

## Pour aller plus loin

[*The Rust Performance Book*](https://nnethercote.github.io/perf-book/) · [Godbolt](https://godbolt.org/) · `criterion`, `divan`, `rayon`
