# 🧩 Petits projets

Huit projets courts (150 à 450 lignes chacun), **autonomes et sans dépendance externe**. Chacun suit la même structure : une **bibliothèque testée** (`src/lib.rs`) et un **programme** (`src/main.rs`) qui l'utilise. Ils servent à pratiquer après les chapitres correspondants, puis à **être étendus** : chaque section se termine par des défis.

```bash
cargo run -p <projet>        # lancer
cargo test -p <projet>       # tests du projet
```

| Projet | Après le chapitre | Vous pratiquez | Lancer |
|:--|:-:|:--|:--|
| [`morpion`](#1-morpion--ia-minimax) | 4 | `enum`, récursion, algorithme minimax | `cargo run -p morpion` |
| [`labyrinthe`](#2-labyrinthe--dfs-et-bfs) | 5 | grilles, `VecDeque`, parcours de graphes | `cargo run -p labyrinthe -- 30 12 7 --solution` |
| [`jeu-de-la-vie`](#3-jeu-de-la-vie) | 5 | `Vec<bool>`, arithmétique modulaire, motifs | `cargo run -p jeu-de-la-vie -- r-pentomino 200 60` |
| [`todo-cli`](#4-todo-cli--gestionnaire-de-tâches) | 6 | erreurs typées, persistance, lib pure + `main` mince | `cargo run -q -p todo-cli -- lister` |
| [`chiffrements`](#5-chiffrements--classiques-et-cryptanalyse) | 7 | itérateurs, `rem_euclid`, statistiques | `cargo run -q -p chiffrements -- rot13 "Bonjour"` |
| [`json-mini`](#6-json-mini--un-parseur-à-la-main) | 9 | descente récursive, `Result`, Unicode, `Display` | `echo '{"a":1}' \| cargo run -q -p json-mini` |
| [`huffman`](#7-huffman--compression-sans-perte) | 10 | `BinaryHeap`, arène, bits, format binaire | `cargo run -p huffman` |
| [`mini-serveur-http`](#8-mini-serveur-http) | 13 | TCP, `ThreadPool`, sécurité web, tests réseau | `cargo run -p mini-serveur-http` |

> Ordre conseillé : tel que listé, du plus simple au plus riche. `mini-serveur-http` réutilise le projet [`thread-pool`](../projets/thread-pool) (chapitre 13).

---

## 1. Morpion — IA minimax

**Code** : [`petits-projets/morpion`](../petits-projets/morpion) · **Durée** : 1 h

Jouez contre une IA qui explore **toutes** les parties possibles (algorithme *negamax* avec élagage alpha-bêta) : elle ne peut pas perdre. Un test le **prouve par exhaustion** : quoi que joue le premier joueur, l'IA en second ne perd jamais.

**À retenir** : modéliser un état de jeu avec des `enum` ; `Option<Joueur>` plutôt qu'un caractère magique ; pourquoi l'IA préfère une victoire rapide (`10 - profondeur`).

**Défis**
1. Ajoutez un niveau « facile » (coup aléatoire une fois sur deux).
2. Généralisez à un plateau N×N avec alignement de K pions (attention : il faudra limiter la profondeur).
3. Comptez le nombre de positions explorées avec et sans élagage alpha-bêta.

## 2. Labyrinthe — DFS et BFS

**Code** : [`petits-projets/labyrinthe`](../petits-projets/labyrinthe) · **Durée** : 1 h 30

Génère un labyrinthe *parfait* (un seul chemin entre deux points) par *recursive backtracker* avec une **pile explicite** (pas de récursion → pas de débordement de pile), puis le résout par un **BFS** (plus court chemin garanti). La graine rend tout reproductible ; un test vérifie la propriété mathématique « arbre couvrant » : `2n − 1` cases ouvertes pour `n` cellules.

**À retenir** : grille plate indexée `y * largeur + x` ; `wrapping_sub` pour éviter un dépassement sur `usize` ; un petit générateur pseudo-aléatoire (xorshift) en 8 lignes.

**Défis**
1. Ajoutez un algorithme de génération différent (Prim, Kruskal) et comparez l'aspect des labyrinthes.
2. Implémentez A* et comparez le nombre de cases visitées avec le BFS.
3. Créez des « boucles » en abattant quelques murs au hasard : le labyrinthe n'est plus parfait, le BFS donne toujours le plus court chemin.

## 3. Jeu de la vie

**Code** : [`petits-projets/jeu-de-la-vie`](../petits-projets/jeu-de-la-vie) · **Durée** : 1 h

Quatre règles, un univers : blocs stables, clignotants, « planeurs » qui traversent la grille. Les tests vérifient des **faits connus** : le bloc est stable, le clignotant a une période 2, un planeur avance d'une case en diagonale tous les 4 pas et **revient à son point de départ** après `4 × largeur` pas (grille torique).

**À retenir** : modulo pour les bords toriques sans nombres négatifs (`x + largeur - 1`) ; produire une nouvelle génération sans modifier l'ancienne ; séquences ANSI pour animer le terminal.

**Défis**
1. Chargez un motif depuis un fichier au format RLE (celui du wiki LifeWiki).
2. Détectez automatiquement qu'une configuration est devenue **stable** ou **périodique** (indice : un `HashSet<Grille>` d'états vus).
3. Mesurez le temps d'une génération sur une grille 2000×2000, puis accélérez avec `rayon` ou des `u64` représentant 64 cellules (*bitboards*).

## 4. `todo-cli` — gestionnaire de tâches

**Code** : [`petits-projets/todo-cli`](../petits-projets/todo-cli) · **Durée** : 1 h 30

Gestionnaire de tâches persistant dans un fichier texte (`id⇥x|-⇥texte`). **Toute la logique est pure** : la bibliothèque transforme du texte en `Liste` et inversement ; seul `main.rs` lit/écrit le disque, avec une **écriture atomique** (fichier temporaire + `rename`).

**À retenir** : la bonne séparation « cœur pur / coquille d'E/S » rend tout testable ; échappement des tabulations et sauts de ligne ; validation d'un fichier corrompu (identifiants en double, dépassement) avec le numéro de ligne.

**Défis**
1. Ajoutez des **dates d'échéance** et un tri par urgence.
2. Ajoutez des **étiquettes** (`#travail`) et un filtre `lister --etiquette travail`.
3. Remplacez le format maison par du JSON avec le projet `json-mini` (ou `serde_json`).

## 5. Chiffrements — classiques et cryptanalyse

**Code** : [`petits-projets/chiffrements`](../petits-projets/chiffrements) · **Durée** : 1 h 30

César, ROT13, Vigenère (testé sur le vecteur classique `ATTACKATDAWN` + `LEMON` → `LXFOPVEFRNHR`), XOR avec encodage hexadécimal, et **cryptanalyse** : un test du χ² sur les fréquences des lettres du français casse automatiquement un César.

> ⚠ **À but pédagogique uniquement** : ces chiffrements sont trivialement cassables. Pour de vraies données : AES-GCM ou ChaCha20-Poly1305 via des bibliothèques auditées. N'écrivez jamais votre propre cryptographie en production.

**À retenir** : `rem_euclid` pour un modulo toujours positif ; l'**indice de coïncidence** montre pourquoi Vigenère résiste à l'analyse simple de fréquences ; un masque jetable XOR n'est sûr que si la clé est aléatoire, secrète, aussi longue que le message et **jamais réutilisée**.

**Défis**
1. Cassez un **Vigenère** : estimez la longueur de clé (indice de coïncidence par colonnes), puis cassez chaque colonne comme un César.
2. Ajoutez la gestion des lettres accentuées (é, è, ç…).
3. Implémentez un chiffre de **Playfair** ou un chiffre par substitution aléatoire, puis cassez-le par analyse de fréquences + recuit simulé.

## 6. `json-mini` — un parseur à la main

**Code** : [`petits-projets/json-mini`](../petits-projets/json-mini) · **Durée** : 2 h

Un analyseur JSON complet (grammaire de json.org) : échappements, `\uXXXX` **avec paires de substitution** (🦀 = `\ud83e\udd80`), nombres stricts (`01` et `1.` refusés), erreurs avec **ligne et colonne**, sérialisation compacte et indentée, accès par chemin (`auteurs.0.nom`).

**À retenir** : une **profondeur maximale** protège d'un `[[[[…` qui ferait déborder la pile (un test le vérifie avec 10 000 niveaux) ; travailler en octets mais avancer par caractères UTF-8 ; la différence entre analyser (`&str` → arbre) et sérialiser (arbre → `String`).

**Défis**
1. Rendre `Json` convertible depuis des types Rust : `impl From<i32> for Json`, `From<&str>`, `From<Vec<T>>`, et une macro `json!{…}`.
2. Écrire un trait `ToJson` et un `derive` (liaison avec le chapitre 20).
3. Ajouter un mode « flux » qui n'alloue pas l'arbre (événements `DebutObjet`, `Cle`, `Valeur`…).

## 7. Huffman — compression sans perte

**Code** : [`petits-projets/huffman`](../petits-projets/huffman) · **Durée** : 2 h

Compresse et décompresse n'importe quel fichier. Le décodeur **reconstruit le même arbre** à partir de la table de fréquences stockée dans l'en-tête (la construction est déterministe : égalités départagées par identifiant). Le texte français descend à ~54 % de sa taille, proche de la limite théorique donnée par l'**entropie de Shannon** (testée : Huffman ≤ H + 1 bit/symbole).

**À retenir** : arène de nœuds (`Vec<Noeud>` + indices) plutôt que `Box` récursifs ; `BinaryHeap` avec `Reverse` en tas-min ; **défense contre un en-tête malveillant** (taille annoncée de 4 Gio → rejet sans allocation) ; cas limites (vide, un seul symbole).

**Défis**
1. Stockez l'arbre sous forme canonique (longueurs de codes seulement) pour réduire l'en-tête.
2. Compressez en **flux** (par blocs de 64 Kio) au lieu de tout charger en mémoire.
3. Comparez avec **LZ77** (étude de `deflate`) puis combinez les deux : vous venez d'inventer DEFLATE.

## 8. `mini-serveur-http`

**Code** : [`petits-projets/mini-serveur-http`](../petits-projets/mini-serveur-http) · **Durée** : 2 h

Un serveur HTTP/1.1 qui réutilise le `ThreadPool` du chapitre 13. Routes : `/`, `/bonjour/<nom>`, `/heure`, `/echo?texte=…`, `/somme?a=…&b=…`. L'analyse, le routage et la sérialisation sont séparés du réseau (donc testables sans socket) ; des tests d'intégration lancent un **vrai serveur** sur un port libre.

**Sécurité abordée** (à lire dans le code) :
- **échappement HTML** de toute donnée utilisateur (sinon *XSS*) — un test injecte `<script>` ;
- **limites** : taille d'une ligne (8 Kio), nombre d'en-têtes, délais de lecture/écriture ;
- **décodage des `%xx`** sans panic sur les séquences invalides ;
- ne jamais fermer brutalement une connexion qu'on n'a pas fini de lire (sinon *reset* côté client : un test de requête géante l'a révélé) ;
- écoute sur `127.0.0.1` uniquement.

**Défis**
1. Servir des **fichiers statiques** d'un dossier — en bloquant la remontée de répertoires (`..`, liens symboliques) : c'est la faille *path traversal*.
2. Gérer le *keep-alive* et les requêtes `POST` avec `Content-Length`.
3. Réécrire le serveur en asynchrone avec `tokio` et comparer (chapitre 14).

---

## Pour aller plus loin

- Reliez-les : `todo-cli` + `json-mini` (stockage JSON), `mini-serveur-http` + `json-mini` (API JSON), `huffman` pour compresser les réponses.
- Ajoutez-y des **benchmarks** (`criterion`) et des **tests par propriétés** (`proptest`), voir chapitres 12 et 19.
- Publiez l'un d'eux sur crates.io, ou écrivez un article sur ce que vous avez appris.
