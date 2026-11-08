# 🗺️ Feuille de route — 16 semaines

Rythme supposé : **6 à 8 heures par semaine**. Doublez les semaines si vous avez moins de temps ; l'essentiel est la **régularité** (30 minutes par jour valent mieux qu'une journée par mois).

Chaque semaine : *lire le cours → exécuter et modifier le code → exercices → mini-défi*.

## 🟢 Phase 1 — Les fondations (semaines 1-3)

| Sem. | Chapitres | Exercices | Mini-défi |
|:-:|:--|:--|:--|
| 1 | 1 Bases · 2 Contrôle | niveau 1 : 1.1 | Convertisseur de températures, jeu « devinez le nombre » |
| 2 | 3 **Ownership** · 4 Structs/enums | niveau 1 : 1.2, 1.3, 1.6 | Gestion de contacts en mémoire (struct + `Vec`) |
| 3 | 5 Collections · 6 Erreurs | niveau 1 : 1.4, 1.5 | Compteur de mots d'un fichier avec gestion d'erreurs |

**Jalon** : écrire un petit programme en ligne de commande sans copier-coller. Lire un message du compilateur sans panique.

## 🟡 Phase 2 — Concevoir en Rust (semaines 4-7)

| Sem. | Chapitres | Exercices | Mini-défi |
|:-:|:--|:--|:--|
| 4 | 7 Traits/génériques | niveau 2 : 2.2, 2.5 | Une bibliothèque de formes géométriques |
| 5 | 8 Lifetimes · 9 Closures/itérateurs | niveau 2 : 2.1, 2.3, 2.4 | Un parseur CSV qui emprunte le texte d'origine |
| 6 | 10 Smart pointers · 11 Modules | niveau 2 : 2.6 | Découper un projet en lib + bin |
| 7 | 12 Tests + **projet `minigrep`** | — | Ajoutez 3 options à `minigrep` avec leurs tests |

**Jalon** : un projet de 300+ lignes, découpé en modules, testé, avec `cargo clippy` sans avertissement.

## 🟠 Phase 3 — Systèmes et abstractions (semaines 8-11)

| Sem. | Chapitres | Exercices | Mini-défi |
|:-:|:--|:--|:--|
| 8 | 13 Concurrence | niveau 3 : 3.1, 3.3, 3.4 | Un téléchargeur multi-thread |
| 9 | 14 Async | — | Un mini-serveur avec Tokio (écho TCP) |
| 10 | 15 Traits avancés · 16 Macros | niveau 3 : 3.2 | Votre propre `hashmap!`, un typestate de connexion |
| 11 | 17 Idiomes + **projet `calculatrice`** | — | Ajoutez des variables, fonctions utilisateur, comparaisons |

**Jalon** : comprendre `Send`/`Sync`, savoir choisir entre threads et async, lire du code Rust idiomatique d'une crate populaire.

## 🔴 Phase 4 — Maîtrise (semaines 12-16)

| Sem. | Chapitres | Exercices | Mini-défi |
|:-:|:--|:--|:--|
| 12 | 18 Unsafe | niveau 4 : 4.1 | Étendre `MonVec` (insert/remove), tests sous Miri |
| 13 | 19 Performance | — | Profiler et accélérer un de vos programmes de 2× |
| 14 | 20 Macros procédurales | — | `#[derive(...)]` avec `syn` et `quote` |
| 15 | 21 Types avancés | niveau 4 : 4.2, 4.3 | Une bibliothèque d'algèbre linéaire à dimensions statiques |
| 16 | **projet `thread-pool`** + livre *Rust for Rustaceans* | — | Publiez une crate sur crates.io |

**Jalon** : contribuer à un projet open source ; lire du code `unsafe` et en juger la solidité.

## Petits projets à intercaler

Les huit projets de [`PETITS_PROJETS.md`](PETITS_PROJETS.md) se glissent dans le parcours : `morpion` (après la sem. 2), `labyrinthe` et `jeu-de-la-vie` (sem. 3), `todo-cli` (sem. 3-4), `chiffrements` (sem. 4), `json-mini` (sem. 5-6), `huffman` (sem. 6-7), `mini-serveur-http` (sem. 8-9).

## Projets de synthèse (au choix)

| Projet | Compétences |
|:--|:--|
| Clone de `ls` / `cat` / `find` en CLI avec `clap` | I/O, erreurs, tests |
| Serveur HTTP avec `axum` + base `sqlx` | async, web, SQL |
| Interpréteur d'un petit langage (suite de la calculatrice) | parsing, enums, récursion |
| Éditeur de texte en terminal avec `ratatui` | état, événements |
| Jeu 2D avec `bevy` | ECS, performance |
| Application WebAssembly (`leptos`/`yew`) | WASM, front-end |
| Pilote ou firmware embarqué (RP2040, `embassy`) | `no_std`, hardware |
| Tableau de supervision industriel (acquisition de données, Modbus/OPC UA) | async, protocoles, fiabilité |

## Conseils pour tenir sur la durée

1. **Tenez un journal** d'erreurs de compilation rencontrées et de leur cause.
2. **Relisez le chapitre 3** à la fin de chaque phase.
3. **Lisez du code** : `ripgrep`, `bat`, `fd`, `tokio` sont d'excellentes lectures.
4. **Posez des questions** (forum, Discord) : la communauté Rust est réputée accueillante.
5. **Enseignez** : écrire un article ou expliquer à quelqu'un révèle ce que vous n'avez pas compris.
6. Ne cherchez pas à « finir » : on apprend Rust par couches.

## Auto-évaluation

Vous êtes **débutant** si vous savez expliquer *move* vs *copy* vs *borrow*.
Vous êtes **intermédiaire** si vous concevez une API avec traits et gérez les erreurs avec des types dédiés.
Vous êtes **avancé** si vous choisissez sciemment entre `Arc<Mutex<T>>`, canaux, atomiques et async.
Vous êtes **expert** si vous pouvez justifier chaque bloc `unsafe` par un invariant écrit, et mesurer avant d'optimiser.
