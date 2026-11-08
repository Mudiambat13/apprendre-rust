#!/usr/bin/env python3
"""Génère la crate `exercices/` (à trous) à partir de la crate `solutions/`.

Règle : tout ce qui se trouve entre une ligne `// SOLUTION` et une ligne `// FIN SOLUTION`
est remplacé par un unique `todo!("…")` (avec la même indentation).

Usage (depuis la racine du dépôt) :
    python3 scripts/generer_exercices.py

À lancer après chaque modification de `solutions/src/*.rs`. Les fichiers de `exercices/src/`
sont ÉCRASÉS : c'est volontaire, `solutions/` est la source de vérité.
"""

from pathlib import Path
import re
import sys

RACINE = Path(__file__).resolve().parent.parent
SOLUTIONS = RACINE / "solutions" / "src"
EXERCICES = RACINE / "exercices" / "src"

DEBUT = re.compile(r"^(\s*)// SOLUTION\s*$")
FIN = re.compile(r"^\s*// FIN SOLUTION\s*$")

AUTORISATIONS = (
    "#![allow(unused_variables, unused_mut, unused_imports, dead_code, unreachable_code)]\n"
    "// ↑ Ces avertissements disparaîtront naturellement quand vous aurez rempli les `todo!()`.\n"
)


def transformer(source: str, nom: str) -> str:
    sortie: list[str] = []
    dans_solution = False
    for numero, ligne in enumerate(source.splitlines(), start=1):
        correspondance = DEBUT.match(ligne)
        if correspondance and not dans_solution:
            dans_solution = True
            sortie.append(f'{correspondance.group(1)}todo!("à vous de jouer !")')
            continue
        if FIN.match(ligne):
            if not dans_solution:
                sys.exit(f"{nom}:{numero}: « FIN SOLUTION » sans « SOLUTION »")
            dans_solution = False
            continue
        if not dans_solution:
            sortie.append(ligne)
    if dans_solution:
        sys.exit(f"{nom}: bloc « SOLUTION » jamais refermé")
    return "\n".join(sortie) + "\n"


def main() -> None:
    EXERCICES.mkdir(parents=True, exist_ok=True)
    for source in sorted(SOLUTIONS.glob("*.rs")):
        texte = source.read_text(encoding="utf-8")
        if source.name == "lib.rs":
            texte = texte.replace(
                "//! # Corrigés\n//!\n"
                "//! Ne regardez ce dossier qu'**après** avoir essayé l'exercice correspondant dans `exercices/`.\n//!\n"
                "//! La crate `exercices` est générée à partir de celle-ci par `scripts/generer_exercices.py` :\n"
                "//! le code compris entre `// SOLUTION` et `// FIN SOLUTION` y est remplacé par `todo!()`.\n",
                "//! # Exercices\n//!\n"
                "//! Ouvrez `src/niveau1.rs`, remplacez les `todo!()` par votre code, puis lancez\n"
                "//! `cargo test -p exercices` (les tests échouent tant que ce n'est pas fait).\n"
                "//! Corrigés : dossier `solutions/`.\n",
            )
            texte = AUTORISATIONS + texte
        resultat = transformer(texte, source.name)
        (EXERCICES / source.name).write_text(resultat, encoding="utf-8")
        print(f"✔ exercices/src/{source.name}")


if __name__ == "__main__":
    main()
