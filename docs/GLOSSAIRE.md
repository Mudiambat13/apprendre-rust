# 📖 Glossaire

| Terme | Définition |
|:--|:--|
| **Ownership** (propriété) | Règle : chaque valeur a un unique propriétaire ; à sa sortie de portée la valeur est détruite. |
| **Move** (déplacement) | Transfert de propriété ; l'ancienne variable n'est plus utilisable. |
| **Copy** | Trait des types dupliqués bit à bit à l'affectation (`i32`, `bool`, `&T`…). |
| **Clone** | Duplication explicite, éventuellement coûteuse. |
| **Borrow** (emprunt) | Référence temporaire (`&T` ou `&mut T`) sans prise de propriété. |
| **Borrow checker** | Partie du compilateur qui vérifie les règles d'emprunt. |
| **NLL** | *Non-Lexical Lifetimes* : un emprunt dure jusqu'à sa dernière utilisation. |
| **Lifetime** (durée de vie) | Région pendant laquelle une référence est valide, notée `'a`. |
| **Élision** | Déduction automatique des lifetimes par des règles fixes. |
| **Slice** | Vue (pointeur + longueur) sur une suite : `&[T]`, `&str`. |
| **Trait** | Ensemble de comportements qu'un type peut implémenter (≈ interface). |
| **Trait object** | `dyn Trait` : valeur dont le type concret est inconnu à la compilation (dispatch dynamique). |
| **Monomorphisation** | Génération d'une copie de code par type concret d'un générique. |
| **Blanket impl** | Implémentation d'un trait pour *tous* les types respectant une contrainte. |
| **Orphan rule** | On ne peut implémenter un trait que si le trait ou le type est local à la crate. |
| **Newtype** | Struct à un champ servant à créer un type distinct (`struct Euros(f64)`). |
| **Typestate** | Encodage de l'état d'un objet dans son type. |
| **Closure** | Fonction anonyme capturant son environnement (`Fn`, `FnMut`, `FnOnce`). |
| **Iterator** | Trait à une méthode (`next`) offrant de nombreux adaptateurs paresseux. |
| **Smart pointer** | Type se comportant comme un pointeur et gérant une ressource (`Box`, `Rc`, `Arc`). |
| **RAII** | Acquisition = initialisation ; libération automatique via `Drop`. |
| **Interior mutability** | Mutation à travers `&T` (`Cell`, `RefCell`, `Mutex`, atomiques). |
| **Send / Sync** | Traits auto : déplaçable / partageable entre threads. |
| **Data race** | Accès concurrents non synchronisés dont l'un écrit ; impossible en Rust sûr. |
| **Deadlock** | Blocage mutuel de threads attendant chacun un verrou de l'autre. |
| **Future** | Valeur représentant un calcul asynchrone, avancée par `poll`. |
| **Waker** | Mécanisme par lequel un événement réveille l'exécuteur pour re-sonder un futur. |
| **Executor / runtime** | Composant qui sonde les futurs (`tokio`, `smol`). |
| **Pin** | Garantie qu'une valeur ne sera plus déplacée en mémoire. |
| **Macro déclarative** | `macro_rules!` : filtrage de motifs sur des jetons. |
| **Macro procédurale** | Fonction exécutée à la compilation, jetons → jetons (`derive`, attribut, fonction). |
| **`TokenStream`** | Flux de jetons manipulé par les macros. |
| **Hygiène** | Isolation des variables locales d'une macro vis-à-vis de l'appelant. |
| **`unsafe`** | Bloc où le programmeur garantit des invariants que le compilateur ne vérifie pas. |
| **UB** | *Undefined Behavior* : comportement indéfini ; le programme n'a plus de sens. |
| **FFI** | *Foreign Function Interface* : appels vers/depuis d'autres langages (C…). |
| **Niche** | Valeur interdite d'un type réutilisée pour encoder `None` sans surcoût. |
| **Zero-cost abstraction** | Abstraction n'entraînant aucun surcoût à l'exécution. |
| **Crate** | Unité de compilation (bibliothèque ou binaire). |
| **Package** | Projet Cargo (`Cargo.toml`). |
| **Workspace** | Ensemble de packages partageant `Cargo.lock` et `target/`. |
| **Edition** | Version du langage compatible en interopérabilité (2015, 2018, 2021, 2024). |
| **Feature** | Option de compilation d'une crate (`[features]`). |
| **Doctest** | Exemple de code d'un commentaire `///` exécuté comme test. |
| **`no_std`** | Mode sans bibliothèque standard (embarqué, noyaux). |
| **MIR / LLVM** | Représentations intermédiaires du compilateur ; LLVM génère le code machine. |
| **Miri** | Interpréteur détectant le comportement indéfini. |
