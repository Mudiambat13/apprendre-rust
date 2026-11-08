# Chapitre 18 — `unsafe` Rust

> 🔴 Expert · Code : [`src/bin/ch18_unsafe.rs`](../../src/bin/ch18_unsafe.rs) · vérification : `cargo +nightly miri test --bin ch18_unsafe`

## Ce qu'`unsafe` fait — et ne fait pas

`unsafe` **ne désactive pas** le borrow checker. Il débloque cinq capacités :

1. déréférencer un pointeur brut (`*const T`, `*mut T`) ;
2. appeler une fonction `unsafe` (dont les fonctions FFI) ;
3. accéder à une `static mut` (à éviter) ;
4. implémenter un trait `unsafe` (`Send`, `Sync`) ;
5. accéder aux champs d'une `union`.

Dans l'édition 2024 : les blocs `extern` s'écrivent `unsafe extern "C" { … }` et le lint `unsafe_op_in_unsafe_fn` est actif par défaut : même dans une `unsafe fn`, les opérations dangereuses doivent être placées dans un bloc `unsafe {}` explicite.

## Le contrat

Vous promettez au compilateur que **certains invariants tiennent**, qu'il ne peut pas vérifier. Violer la promesse = **comportement indéfini** (UB) : tout peut arriver, y compris « ça marche en debug mais plante en release ».

Comportements indéfinis typiques : déréférencer un pointeur nul/pendant/mal aligné, créer deux `&mut` vers la même donnée, lire de la mémoire non initialisée, produire un `bool`/`char`/`enum` invalide, data race, violer les règles d'aliasing.

## La méthode

1. **Minimisez** la surface `unsafe` (quelques lignes, dans un module).
2. **Encapsulez** dans une API **sûre** (*sound*) : aucune utilisation depuis le code sûr ne doit pouvoir causer d'UB.
3. **Documentez** chaque bloc par `// SAFETY: …` (pourquoi les invariants tiennent), et chaque `unsafe fn` par une section `# Safety`.
4. **Testez sous Miri** et, si possible, avec des sanitizers.

## Au programme du fichier

| Section | Idée |
|:--|:--|
| Pointeurs bruts | créer = sûr, déréférencer = `unsafe` |
| `diviser_en_deux` | réimplémenter `split_at_mut` : deux `&mut` disjoints |
| FFI | appeler `abs`/`strlen` de la libc ; `CString` ; `unsafe extern` |
| `MonVec<T>` | un `Vec` complet : `alloc`, `realloc`, `dealloc`, `ptr::read/write`, `NonNull`, `Drop`, `Deref` |
| `unsafe impl Send/Sync` | affirmer que son type est thread-safe |

## Outils

- **Miri** : interpréteur qui détecte l'UB (`cargo +nightly miri test`).
- **Sanitizers** (nightly) : `-Zsanitizer=address|thread`.
- **`cargo geiger`** : compte l'`unsafe` dans les dépendances.
- **Loom** : teste exhaustivement les entrelacements de threads.

## À retenir

- Si une fonction `unsafe` peut être appelée depuis du code sûr sans précondition, elle doit être **sûre**.
- Si vous doutez, vous n'avez probablement pas besoin d'`unsafe` : cherchez dans `std` ou une crate éprouvée.

## Exercices

1. Niveau 4, ex. 4.1 : `diviser_en_deux_mut`.
2. Ajoutez `insert(index, valeur)` et `remove(index)` à `MonVec` (`ptr::copy`), testez sous Miri.
3. Provoquez volontairement une UB (lecture hors limites via pointeur) et observez le rapport de Miri.

## Pour aller plus loin

[The Book ch. 19.1](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) · [**Rustonomicon**](https://doc.rust-lang.org/nomicon/) · [Unsafe Code Guidelines](https://rust-lang.github.io/unsafe-code-guidelines/) · *Too Many Linked Lists*
