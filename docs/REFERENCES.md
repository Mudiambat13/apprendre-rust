# 📚 Références

Ressources classées par usage. Les liens pointent vers les sites officiels ou les sources de référence ; vérifiez-les avant de les partager, le web évolue.

## 1. Documentation officielle (à connaître absolument)

| Ressource | Usage | Lien |
|:--|:--|:--|
| **The Rust Programming Language** (« The Book ») | Le livre de référence pour apprendre | <https://doc.rust-lang.org/book/> |
| **Rust By Example** | Apprendre par petits exemples exécutables | <https://doc.rust-lang.org/rust-by-example/> |
| **Rustlings** | Petits exercices à corriger dans le terminal | <https://github.com/rust-lang/rustlings> |
| **Standard Library** | La doc de `std` (à garder ouverte en permanence) | <https://doc.rust-lang.org/std/> |
| **The Cargo Book** | Gestionnaire de paquets, workspaces, features | <https://doc.rust-lang.org/cargo/> |
| **The Rust Reference** | Spécification du langage (syntaxe, sémantique) | <https://doc.rust-lang.org/reference/> |
| **The Rustonomicon** | `unsafe`, modèle mémoire, variance, drop check | <https://doc.rust-lang.org/nomicon/> |
| **The Rustdoc Book** | Documenter son code, doctests | <https://doc.rust-lang.org/rustdoc/> |
| **The Edition Guide** | Différences entre éditions 2015/2018/2021/2024 | <https://doc.rust-lang.org/edition-guide/> |
| **Rust Error Codes Index** | Explication de chaque erreur `E0xxx` | <https://doc.rust-lang.org/error_codes/error-index.html> |
| **Async Book** | Programmation asynchrone | <https://rust-lang.github.io/async-book/> |
| **The `rustc` Book** | Options du compilateur, lints, cibles | <https://doc.rust-lang.org/rustc/> |
| **Embedded Rust Book** | Microcontrôleurs et systèmes embarqués | <https://docs.rust-embedded.org/book/> |
| **Rust API Guidelines** | Conventions pour concevoir des API idiomatiques | <https://rust-lang.github.io/api-guidelines/> |
| **Unsafe Code Guidelines** | Ce qui est ou non un comportement indéfini | <https://rust-lang.github.io/unsafe-code-guidelines/> |
| **Rust Playground** | Essayer du code dans le navigateur | <https://play.rust-lang.org/> |

> La documentation de chaque crate est sur <https://docs.rs/NOM_DE_LA_CRATE>.
> Hors ligne : `rustup doc --book`, `rustup doc --std`, `cargo doc --open`.

## 2. Livres

| Titre | Auteur(s) | Niveau | Pourquoi le lire |
|:--|:--|:--|:--|
| *The Rust Programming Language* | Steve Klabnik, Carol Nichols (+ contributeurs) | 🟢🟡 | Le point de départ, gratuit en ligne, édition imprimée chez No Starch Press |
| *Programming Rust* (2e éd.) | Jim Blandy, Jason Orendorff, Leonora Tindall | 🟡🟠 | Très bonne explication du modèle mémoire et des choix de conception |
| *Rust for Rustaceans* | Jon Gjengset | 🟠🔴 | Le meilleur livre « après le Book » : API, traits, async, unsafe, macros |
| *Rust Atomics and Locks* | Mara Bos | 🟠🔴 | Atomiques, ordres mémoire, implémenter `Mutex`, `Condvar`… (gratuit en ligne : <https://marabos.nl/atomics/>) |
| *Zero To Production In Rust* | Luca Palmieri | 🟡🟠 | Construire un service web réel de bout en bout |
| *Rust in Action* | Tim McNamara | 🟡 | Rust par des projets système (réseau, fichiers, CPU) |
| *Command-Line Rust* | Ken Youens-Clark | 🟢🟡 | Réécrire des outils Unix en Rust |
| *Hands-on Rust* | Herbert Wolverson | 🟢 | Apprendre en écrivant un jeu |
| *Learn Rust With Entirely Too Many Linked Lists* | Alexis Beingessner | 🟠🔴 | Maîtriser ownership et `unsafe` via les listes chaînées : <https://rust-unofficial.github.io/too-many-lists/> |
| *Effective Rust* | David Drysdale | 🟡🟠 | 35 conseils pour écrire du Rust idiomatique : <https://www.lurklurk.org/effective-rust/> |
| *Idiomatic Rust* (patterns) | Rust Unofficial | 🟠 | Patterns, anti-patterns, idiomes : <https://rust-unofficial.github.io/patterns/> |
| *The Little Book of Rust Macros* | Daniel Keep et al. | 🟠🔴 | Référence sur `macro_rules!` : <https://veykril.github.io/tlborm/> |
| *Rust Design Patterns* | communauté | 🟠 | Catalogue de patterns : <https://rust-unofficial.github.io/patterns/> |

## 3. Cours vidéo et chaînes

| Ressource | Intérêt |
|:--|:--|
| **Jon Gjengset** — chaîne YouTube (*Crust of Rust*, *Decrusted*) | Sessions longues et approfondies : lifetimes, iterators, atomics, async… |
| **Let's Get Rusty** — chaîne YouTube | Excellent pour débuter, lecture commentée du Book |
| **No Boilerplate** — chaîne YouTube | Vidéos courtes pour comprendre l'intérêt de Rust |
| **Rust Conf / RustFest / EuroRust** — conférences | Talks gratuits sur YouTube, bon moyen de suivre l'écosystème |
| **Comprehensive Rust** (Google) | Cours en 4 jours, traduit en français : <https://google.github.io/comprehensive-rust/> |
| **Exercism — Rust track** | Exercices avec mentorat : <https://exercism.org/tracks/rust> |

## 4. Exercices et entraînement

| Plateforme | Description |
|:--|:--|
| Rustlings | Corriger de petits programmes qui ne compilent pas |
| Exercism | Exercices progressifs, revue de code par des mentors |
| Advent of Code | <https://adventofcode.com/> — énigmes quotidiennes en décembre, parfait pour pratiquer |
| Project Euler | <https://projecteuler.net/> — mathématiques et algorithmique |
| LeetCode / CodeWars | Algorithmique (Rust supporté) |
| *Too Many Linked Lists* | Défi de maîtrise de la mémoire |
| Contribuer à un projet open source | Cherchez les tickets `good first issue` : <https://github.com/rust-lang/rust/labels/E-easy> |

## 5. Outils indispensables

| Outil | Commande | Rôle |
|:--|:--|:--|
| `rustup` | `rustup update` | Gère les versions du compilateur et des composants |
| `cargo fmt` | `cargo fmt --all` | Formate le code (standard de la communauté) |
| `cargo clippy` | `cargo clippy --all-targets` | Détecte des centaines de mauvais usages |
| `rust-analyzer` | (dans l'éditeur) | Autocomplétion, navigation, types inférés : <https://rust-analyzer.github.io/> |
| `cargo test` / `cargo nextest` | `cargo nextest run` | Lancer les tests (nextest : plus rapide) |
| `cargo doc` | `cargo doc --open` | Générer et lire la documentation locale |
| `cargo expand` | `cargo install cargo-expand` | Voir le code généré par les macros |
| `cargo audit` | `cargo install cargo-audit` | Chercher les vulnérabilités connues dans les dépendances |
| `cargo deny` | `cargo install cargo-deny` | Politique de licences et de dépendances |
| `cargo flamegraph` | `cargo install flamegraph` | Profil de performance visuel |
| `criterion` / `divan` | (crates de dev) | Micro-benchmarks statistiques |
| `miri` | `cargo +nightly miri test` | Interprète le code et détecte les comportements indéfinis |
| `cargo-watch` / `bacon` | `bacon` | Recompile et relance les tests à chaque sauvegarde |
| `cargo-machete` | `cargo install cargo-machete` | Détecte les dépendances inutilisées |
| `cargo-llvm-cov` | `cargo install cargo-llvm-cov` | Couverture de tests |
| `cargo-semver-checks` | `cargo install cargo-semver-checks` | Vérifie qu'une release respecte SemVer |

## 6. Crates incontournables

| Domaine | Crates |
|:--|:--|
| Sérialisation | [`serde`](https://serde.rs/), `serde_json`, `toml`, `bincode` |
| Erreurs | [`thiserror`](https://docs.rs/thiserror) (bibliothèques), [`anyhow`](https://docs.rs/anyhow) (applications), `miette` |
| Ligne de commande | [`clap`](https://docs.rs/clap), `indicatif` (barres de progression), `ratatui` (TUI) |
| Async | [`tokio`](https://tokio.rs/), `async-std`, `smol`, `futures` |
| Web serveur | [`axum`](https://docs.rs/axum), `actix-web`, `rocket` |
| Client HTTP | `reqwest`, `ureq` |
| Bases de données | [`sqlx`](https://docs.rs/sqlx), `diesel`, `sea-orm`, `rusqlite` |
| Journalisation | [`tracing`](https://docs.rs/tracing), `log`, `env_logger` |
| Parallélisme de données | [`rayon`](https://docs.rs/rayon), `crossbeam` |
| Macros | `syn`, `quote`, `proc-macro2`, `darling` |
| Tests | `proptest` (tests par propriétés), `insta` (snapshots), `mockall`, `criterion` |
| Aléatoire / temps | `rand`, `chrono`, `time` |
| Parsing | `nom`, `winnow`, `pest`, `regex` |
| Interface graphique | `egui`, `iced`, `tauri`, `slint`, `bevy` (jeux) |
| WebAssembly | `wasm-bindgen`, `wasm-pack`, `trunk`, `leptos`, `yew` |
| FFI | `libc`, `bindgen`, `cbindgen`, `pyo3` (Python), `napi-rs` (Node) |
| Embarqué | `embedded-hal`, `embassy`, `probe-rs` |

Pour comparer des crates : <https://lib.rs/> et <https://crates.io/>. Guide communautaire de choix : <https://blessed.rs/>.

## 7. Communautés (et où poser vos questions)

| Lieu | Lien |
|:--|:--|
| Forum officiel *users.rust-lang.org* | <https://users.rust-lang.org/> |
| Discord communautaire | <https://discord.gg/rust-lang-community> |
| Reddit | <https://www.reddit.com/r/rust/> et <https://www.reddit.com/r/learnrust/> |
| Stack Overflow | étiquette `rust` |
| Francophone | cherchez les groupes locaux *Rust France* / *Rust Africa* et les salons francophones des Discord ci-dessus |
| Newsletter hebdomadaire | **This Week in Rust** : <https://this-week-in-rust.org/> |
| Blog officiel | <https://blog.rust-lang.org/> |
| Suivre les évolutions du langage | RFCs : <https://github.com/rust-lang/rfcs> · suivi des fonctionnalités : <https://github.com/rust-lang/rust/issues> |

## 8. Articles et ressources approfondies

| Thème | Ressource |
|:--|:--|
| Ownership expliqué visuellement | *Brown University Rust Book (version interactive avec quiz)* : <https://rust-book.cs.brown.edu/> |
| Comprendre `Pin` | *Pin and suffering* — fasterthanli.me : <https://fasterthanli.me/articles/pin-and-suffering> |
| Async en profondeur | *Async: What is blocking?* — Alice Ryhl : <https://ryhl.io/blog/async-what-is-blocking/> |
| Les lifetimes en pratique | *Common Rust Lifetime Misconceptions* — pretzelhammer : <https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md> |
| Disposition mémoire | *Rust Container Cheat Sheet* (communauté) : schémas de `Vec`, `Box`, `Rc`, `RefCell`… — cherchez le titre exact |
| Modèle mémoire C++/Rust | cppreference — *memory_order* : <https://en.cppreference.com/w/cpp/atomic/memory_order> |
| Performance | *The Rust Performance Book* — Nicholas Nethercote : <https://nnethercote.github.io/perf-book/> |
| Optimisations du compilateur | *Compiler Explorer (Godbolt)* : <https://godbolt.org/> — lire l'assembleur généré |
| Bonnes pratiques | *Rust API Guidelines Checklist* : <https://rust-lang.github.io/api-guidelines/checklist.html> |
| Écrire des macros procédurales | `syn` guide : <https://docs.rs/syn> · `quote` : <https://docs.rs/quote> · *Procedural Macros Workshop* — David Tolnay : <https://github.com/dtolnay/proc-macro-workshop> |
| Sécurité | *RustSec Advisory Database* : <https://rustsec.org/> · *Secure Rust Guidelines* — ANSSI : <https://anssi-fr.github.io/rust-guide/> |

## 9. Référence par chapitre

| Chapitre | Lire en complément |
|:--|:--|
| 1 — Bases | Book ch. 3 · Rust By Example « Primitives » |
| 2 — Contrôle | Book ch. 3.5-3.6, ch. 6 et 18 (patterns) |
| 3 — Ownership | Book ch. 4 · version Brown (quiz) · *Rustlings* `move_semantics` |
| 4 — Structs/enums | Book ch. 5-6 · Rust By Example « Custom Types » |
| 5 — Collections | Book ch. 8 · doc `std::collections` (choix d'une collection) |
| 6 — Erreurs | Book ch. 9 · `thiserror` et `anyhow` · *Error Handling in Rust* — Andrew Gallant (BurntSushi) |
| 7 — Traits | Book ch. 10 et 18 · *Rust for Rustaceans* ch. 2-3 |
| 8 — Lifetimes | Book ch. 10.3 · Nomicon « Lifetimes » · article de pretzelhammer |
| 9 — Closures/itérateurs | Book ch. 13 · doc `std::iter` |
| 10 — Smart pointers | Book ch. 15 · doc `std::rc`, `std::cell` |
| 11 — Modules | Book ch. 7 · Cargo Book « Workspaces » |
| 12 — Tests | Book ch. 11 · Rustdoc Book « Documentation tests » · `proptest` |
| 13 — Concurrence | Book ch. 16 · *Rust Atomics and Locks* ch. 1-5 |
| 14 — Async | Async Book · Tokio tutorial : <https://tokio.rs/tokio/tutorial> |
| 15 — Traits avancés | Book ch. 19 · *Rust for Rustaceans* ch. 3 |
| 16 — Macros | Little Book of Rust Macros · Book ch. 19.5 |
| 17 — Idiomes | Rust Design Patterns · API Guidelines · *Effective Rust* |
| 18 — Unsafe | Nomicon · Unsafe Code Guidelines · Miri · *Too Many Linked Lists* |
| 19 — Performance | Rust Performance Book · Godbolt · `criterion` · `cargo flamegraph` |
| 20 — Proc macros | Reference « Procedural macros » · `syn`/`quote` · workshop de dtolnay |
| 21 — Types avancés | Reference · Rustonomicon (variance, drop check, `PhantomData`) |

## 10. Pour continuer après ce dépôt

1. **Web** : *Zero To Production* + `axum` + `sqlx`.
2. **Systèmes** : *Rust Atomics and Locks*, puis lire le code de `tokio` ou `crossbeam`.
3. **Embarqué** : Embedded Rust Book, `embassy`, une carte à 10 € (RP2040).
4. **WebAssembly** : `wasm-bindgen` + `leptos`.
5. **Compilateur** : *Rustc Dev Guide* <https://rustc-dev-guide.rust-lang.org/> et contribuer à Rust.
6. **Outils** : écrire sa propre CLI avec `clap` et la publier sur crates.io.
