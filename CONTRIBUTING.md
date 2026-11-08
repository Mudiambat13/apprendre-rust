# Contribuer

Merci de votre intérêt ! Toute amélioration est bienvenue : faute de frappe, clarification, nouvel exemple, nouvel exercice, lien cassé.

## Avant d'ouvrir une *pull request*

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --exclude exercices
python3 scripts/generer_exercices.py     # si vous avez modifié solutions/
cargo check -p exercices --tests
```

Tout doit passer. La CI (`.github/workflows/ci.yml`) vérifie les mêmes points.

## Règles

- **Exemples exécutables** : chaque chapitre a un programme dans `src/bin/chNN_*.rs`. Il doit compiler sans avertissement et se terminer en quelques secondes.
- **Commentaires pédagogiques** : expliquez le *pourquoi*, pas seulement le *quoi*. Si une erreur de compilation est instructive, montrez-la en commentaire.
- **Petits projets** : dans `petits-projets/<nom>/` (lib + bin + tests, sans dépendance externe si possible) ; ajoutez-le au `Cargo.toml` racine et à `docs/PETITS_PROJETS.md`.
- **Exercices** : modifiez `solutions/src/niveau*.rs` (jamais `exercices/` à la main : il est **généré**). Entourez le code à masquer par `// SOLUTION` … `// FIN SOLUTION`.
- **Sources** : ne copiez pas de longs extraits de livres ou d'articles ; citez et liez.
- **Langue** : français, termes anglais entre parenthèses la première fois (*borrow checker*).
- **Édition Rust** : 2024, version minimale indiquée dans `Cargo.toml` (`rust-version`).

## Signaler un problème

Ouvrez une *issue* en précisant : chapitre concerné, version (`rustc --version`), système, message d'erreur complet.
