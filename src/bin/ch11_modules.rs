//! # Chapitre 11 — Modules, visibilité et organisation du code
//!
//! Pour la version « fichiers » (un module = un fichier) lisez `docs/chapitres/11-modules.md`
//! et regardez le projet `projets/minigrep` (lib + bin) ainsi que le workspace racine.
//!
//! Lancer : `cargo run --bin ch11_modules`

#![allow(dead_code)]

// Un module regroupe des éléments et contrôle leur visibilité.
// Tout est PRIVÉ par défaut ; `pub` rend public.
mod restaurant {
    pub mod salle {
        #[derive(Debug)]
        pub struct Table {
            pub numero: u32, // champ public
            places: u32,     // champ privé : seule la fonction `nouvelle` peut le fixer
        }

        impl Table {
            pub fn nouvelle(numero: u32, places: u32) -> Table {
                Table { numero, places }
            }

            pub fn places(&self) -> u32 {
                self.places
            }
        }

        pub fn accueillir(client: &str) -> String {
            // `super::` remonte au module parent.
            format!(
                "{client} est installé ; {}",
                super::cuisine::preparer("plat du jour")
            )
        }
    }

    pub(crate) mod cuisine {
        // pub(crate) : visible dans toute la crate, mais pas à l'extérieur.
        pub fn preparer(plat: &str) -> String {
            format!("on prépare le {plat} ({} secrets de chef)", secrets())
        }

        fn secrets() -> u32 {
            42 // privé : inaccessible hors de `cuisine` (et ses descendants)
        }
    }

    // Les `enum` publics ont des variantes automatiquement publiques.
    #[derive(Debug)]
    pub enum Menu {
        Entree,
        Plat,
        Dessert,
    }
}

// `use` amène des noms dans la portée. `as` renomme. `self` désigne le module lui-même.
use restaurant::Menu as CarteDuJour;
use restaurant::salle;

// `pub use` : ré-exporte pour construire une façade d'API plus simple.
mod api {
    pub use super::restaurant::Menu;
    pub use super::restaurant::salle::accueillir;
}

// Un « prelude » maison : une façon de regrouper les imports fréquents.
mod prelude {
    pub use super::api::*;
    pub use super::restaurant::salle::Table;
}

// Constantes et statiques globales
const NOM_RESTAURANT: &str = "Chez Ferris";
static VERSION: &str = "1.0"; // 'static : un emplacement mémoire unique pour tout le programme

fn main() {
    let t = prelude::Table::nouvelle(7, 4);
    println!(
        "{NOM_RESTAURANT} v{VERSION} — table n°{} ({} places)",
        t.numero,
        t.places()
    );
    // println!("{}", t.places); // ← ERREUR : le champ `places` est privé

    println!("{}", salle::accueillir("Alice"));
    println!("{}", api::accueillir("Bob"));
    println!("{:?} / {:?}", CarteDuJour::Plat, prelude::Menu::Dessert);

    // Chemins : absolu (`crate::…`) ou relatif (`self::…`, `super::…`).
    println!("{}", crate::restaurant::cuisine::preparer("gratin"));
}

// ───────────────────── Rappel : module ↔ fichier ─────────────────────
//
//   src/lib.rs            ← racine de la crate (déclare `mod reseau;`)
//   src/reseau.rs         ← le module `reseau`
//   src/reseau/tcp.rs     ← le sous-module `reseau::tcp` (déclaré par `mod tcp;` dans reseau.rs)
//
// Un workspace Cargo regroupe plusieurs crates qui partagent `Cargo.lock` et `target/`
// (voir le `Cargo.toml` à la racine de ce dépôt).
