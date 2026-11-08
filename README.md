# 🦀 Apprendre Rust — de zéro à expert

Un parcours **complet, progressif et exécutable** pour apprendre Rust en français : 21 chapitres de cours, chacun accompagné d'un programme commenté que vous pouvez lancer, 4 niveaux d'exercices corrigés, 3 projets guidés, 8 petits projets et une bibliographie commentée.

![CI](https://github.com/Mudiambat13/apprendre-rust/actions/workflows/ci.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)
![Edition](https://img.shields.io/badge/edition-2024-blue)
![License](https://img.shields.io/badge/licence-MIT-green)

---

## Démarrage rapide

```bash
# 1. Installer Rust (https://rustup.rs)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Cloner le dépôt
git clone https://github.com/VOTRE-COMPTE/apprendre-rust.git
cd apprendre-rust

# 3. Lancer votre premier chapitre
cargo run --bin ch01_bases

# 4. Lancer les tests (tout doit être vert)
cargo test --workspace --exclude exercices
```

Chaque chapitre = **un fichier de cours** (`docs/chapitres/NN-*.md`) **+ un programme** (`src/bin/chNN_*.rs`).
Lisez le cours, puis lisez et exécutez le code, puis modifiez-le : c'est en cassant le code qu'on apprend.

---

## Le parcours

| Niveau | Chapitres | Vous saurez… | Durée indicative |
|:--|:--|:--|:--|
| 🟢 **Débutant** | 1 → 6 | écrire des programmes complets, comprendre l'ownership, gérer les erreurs | 2-3 semaines |
| 🟡 **Intermédiaire** | 7 → 12 | concevoir des API avec traits, génériques, lifetimes, closures, tester | 3-4 semaines |
| 🟠 **Avancé** | 13 → 17 | écrire du code concurrent, async, des macros, utiliser les idiomes pros | 3-4 semaines |
| 🔴 **Expert** | 18 → 21 | `unsafe`, optimiser, macros procédurales, système de types avancé | 4+ semaines |

### Table des matières

| # | Chapitre | Cours | Code (`cargo run --bin …`) |
|--:|:--|:--|:--|
| 1 | Les bases | [01-bases](docs/chapitres/01-bases.md) | `ch01_bases` |
| 2 | Flux de contrôle | [02-controle](docs/chapitres/02-controle.md) | `ch02_controle` |
| 3 | **Ownership**, emprunts, slices | [03-ownership](docs/chapitres/03-ownership.md) | `ch03_ownership` |
| 4 | Structs, enums, `Option` | [04-structs-enums](docs/chapitres/04-structs-enums.md) | `ch04_structs_enums` |
| 5 | Collections : `Vec`, `String`, `HashMap` | [05-collections](docs/chapitres/05-collections.md) | `ch05_collections` |
| 6 | Gestion des erreurs | [06-erreurs](docs/chapitres/06-erreurs.md) | `ch06_erreurs` |
| 7 | Génériques et traits | [07-traits-generiques](docs/chapitres/07-traits-generiques.md) | `ch07_traits_generiques` |
| 8 | Durées de vie (lifetimes) | [08-lifetimes](docs/chapitres/08-lifetimes.md) | `ch08_lifetimes` |
| 9 | Closures et itérateurs | [09-closures-iterateurs](docs/chapitres/09-closures-iterateurs.md) | `ch09_closures_iterateurs` |
| 10 | Pointeurs intelligents | [10-smart-pointers](docs/chapitres/10-smart-pointers.md) | `ch10_smart_pointers` |
| 11 | Modules et organisation | [11-modules](docs/chapitres/11-modules.md) | `ch11_modules` |
| 12 | Tester son code | [12-tests](docs/chapitres/12-tests.md) | `cargo test --bin ch12_tests` |
| 13 | Concurrence | [13-concurrence](docs/chapitres/13-concurrence.md) | `ch13_concurrence` |
| 14 | Programmation asynchrone | [14-async](docs/chapitres/14-async.md) | `ch14_async` |
| 15 | Traits avancés | [15-traits-avances](docs/chapitres/15-traits-avances.md) | `ch15_traits_avances` |
| 16 | Macros déclaratives | [16-macros](docs/chapitres/16-macros.md) | `ch16_macros` |
| 17 | Patterns et idiomes | [17-patterns-idiomes](docs/chapitres/17-patterns-idiomes.md) | `ch17_patterns_idiomes` |
| 18 | `unsafe` Rust | [18-unsafe](docs/chapitres/18-unsafe.md) | `ch18_unsafe` |
| 19 | Performance | [19-performance](docs/chapitres/19-performance.md) | `cargo run --release --bin ch19_performance` |
| 20 | Macros procédurales | [20-proc-macros](docs/chapitres/20-proc-macros.md) | `ch20_proc_macros` |
| 21 | Système de types avancé | [21-types-avances](docs/chapitres/21-types-avances.md) | `ch21_types_avances` |

Documents complémentaires : [**Petits projets**](docs/PETITS_PROJETS.md) · [**Feuille de route**](docs/FEUILLE_DE_ROUTE.md) · [**Références**](docs/REFERENCES.md) · [**Glossaire**](docs/GLOSSAIRE.md) · [**Aide-mémoire**](docs/AIDE_MEMOIRE.md) · [**Pièges courants**](docs/PIEGES_COURANTS.md) · [**Contribuer**](CONTRIBUTING.md)

---

## Exercices (avec corrigés)

Le dossier [`exercices/`](exercices/) contient des fonctions à compléter (`todo!()`) avec des tests prêts à l'emploi. Votre but : **faire passer tous les tests au vert**.

```bash
cargo test -p exercices niveau1      # 6 exercices : FizzBuzz, palindrome, CSV, fréquences…
cargo test -p exercices niveau2      # lifetimes, pile générique, closures, traits, liste chaînée
cargo test -p exercices niveau3      # threads, opérateurs, atomiques, map/reduce parallèle
cargo test -p exercices niveau4      # unsafe, génériques constants, Drop
```

Les corrigés sont dans [`solutions/`](solutions/) — **essayez d'abord, regardez ensuite**. Le dossier `exercices/` est *généré* depuis `solutions/` par `python3 scripts/generer_exercices.py`, donc les deux ne peuvent pas diverger.

## Projets

| Projet | Ce que vous pratiquez | Lancer |
|:--|:--|:--|
| [`minigrep`](projets/minigrep) | lib + bin, arguments, fichiers, erreurs, lifetimes, tests | `cargo run -p minigrep -- -n rust projets/minigrep/exemple.txt` |
| [`calculatrice`](projets/calculatrice) | lexer, parser récursif descendant, AST, évaluateur, REPL | `cargo run -p calculatrice` |
| [`thread-pool`](projets/thread-pool) | threads, canaux, `Arc<Mutex<…>>`, `Drop`, arrêt propre | `cargo run --release -p thread-pool` |

## Petits projets

Huit projets courts, autonomes et sans dépendance, chacun avec ses tests et des défis d'extension — détails dans [`docs/PETITS_PROJETS.md`](docs/PETITS_PROJETS.md).

| Projet | Idée | Lancer |
|:--|:--|:--|
| [`morpion`](petits-projets/morpion) | IA minimax imbattable | `cargo run -p morpion` |
| [`labyrinthe`](petits-projets/labyrinthe) | génération (DFS) + résolution (BFS) | `cargo run -p labyrinthe -- 30 12 7 --solution` |
| [`jeu-de-la-vie`](petits-projets/jeu-de-la-vie) | automate de Conway animé | `cargo run -p jeu-de-la-vie -- r-pentomino 200 60` |
| [`todo-cli`](petits-projets/todo-cli) | gestionnaire de tâches persistant | `cargo run -q -p todo-cli -- ajouter Mon idée` |
| [`chiffrements`](petits-projets/chiffrements) | César, Vigenère, XOR + cryptanalyse | `cargo run -q -p chiffrements -- rot13 "Bonjour"` |
| [`json-mini`](petits-projets/json-mini) | parseur JSON écrit à la main | `echo '{"a":[1,2]}' \| cargo run -q -p json-mini` |
| [`huffman`](petits-projets/huffman) | compresseur sans perte | `cargo run -p huffman` |
| [`mini-serveur-http`](petits-projets/mini-serveur-http) | serveur HTTP multi-thread | `cargo run -p mini-serveur-http` |

## Organisation du dépôt

```
apprendre-rust/
├── README.md                  ← vous êtes ici
├── docs/
│   ├── chapitres/             ← les 21 cours (Markdown)
│   ├── FEUILLE_DE_ROUTE.md    ← plan d'étude semaine par semaine
│   ├── REFERENCES.md          ← livres, docs, outils, communautés
│   ├── GLOSSAIRE.md · AIDE_MEMOIRE.md · PIEGES_COURANTS.md
├── src/
│   ├── lib.rs                 ← bibliothèque (tests unitaires + doctests)
│   └── bin/chNN_*.rs          ← un programme exécutable par chapitre
├── tests/                     ← tests d'intégration
├── exercices/                 ← à compléter (généré)
├── solutions/                 ← corrigés
├── projets/                   ← 3 projets guidés
├── petits-projets/            ← 8 petits projets autonomes (morpion, JSON, Huffman, serveur HTTP…)
├── crates/macros_demo/        ← crate de macros procédurales (chapitre 20)
├── scripts/                   ← génération des exercices
└── .github/workflows/ci.yml   ← intégration continue
```

## Comment étudier efficacement

1. **Lisez le cours**, mais ne cherchez pas à tout retenir du premier coup.
2. **Lancez le programme du chapitre**, puis lisez son code : il est abondamment commenté.
3. **Modifiez-le** : provoquez volontairement les erreurs de compilation décrites dans les commentaires. Lire les messages du compilateur *est* l'apprentissage.
4. **Faites les exercices** du niveau correspondant.
5. **Construisez** : un projet personnel dès le chapitre 6 vaut dix chapitres de lecture.
6. **Revenez** sur l'ownership (ch. 3) et les traits (ch. 7) après les chapitres suivants : ils prennent tout leur sens plus tard.

## Vérifier la qualité du dépôt

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --exclude exercices
cargo +nightly miri test --bin ch18_unsafe     # détecte les comportements indéfinis
```

## Licence

[MIT](LICENSE). Le contenu est original ; les références renvoient vers les ressources officielles et communautaires, qui restent la propriété de leurs auteurs.
