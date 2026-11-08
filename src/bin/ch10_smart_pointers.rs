//! # Chapitre 10 — Pointeurs intelligents
//!
//! `Box<T>`, `Deref`, `Drop`, `Rc<T>`, `RefCell<T>`, `Weak<T>`, `Cell<T>`.
//!
//! Lancer : `cargo run --bin ch10_smart_pointers`

use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    box_demo();
    deref_demo();
    drop_demo();
    rc_demo();
    refcell_demo();
    arbre_demo();
}

// ───────────────────────── Box<T> ─────────────────────────

/// Sans `Box`, ce type aurait une taille infinie (le compilateur refuse).
/// `Box` ajoute une indirection de taille fixe (un pointeur) vers le tas.
#[derive(Debug)]
enum Liste {
    Cons(i32, Box<Liste>),
    Nil,
}

use Liste::{Cons, Nil};

impl Liste {
    fn somme(&self) -> i32 {
        match self {
            Cons(v, suite) => v + suite.somme(),
            Nil => 0,
        }
    }
}

fn box_demo() {
    println!("--- Box ---");
    let b = Box::new(5); // valeur sur le tas
    println!("b = {b}, *b + 1 = {}", *b + 1);

    let liste = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{liste:?} somme = {}", liste.somme());

    // Box<dyn Trait> : objets-traits (voir chapitre 7)
    let affichables: Vec<Box<dyn std::fmt::Display>> =
        vec![Box::new(42), Box::new("texte"), Box::new(2.5)];
    for a in &affichables {
        print!("{a} ");
    }
    println!();
}

// ───────────────────────── Deref ─────────────────────────

struct MaBoite<T>(T);

impl<T> Deref for MaBoite<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn saluer(nom: &str) {
    println!("Bonjour {nom} !");
}

fn deref_demo() {
    println!("--- Deref ---");
    let x = 5;
    let y = MaBoite(x);
    assert_eq!(5, *y); // *y  ≡  *(y.deref())

    // Coercition déréférencante : &MaBoite<String> → &String → &str
    let nom = MaBoite(String::from("Rust"));
    saluer(&nom);
}

// ───────────────────────── Drop ─────────────────────────

struct Ressource(&'static str);

impl Drop for Ressource {
    fn drop(&mut self) {
        println!("  libération de {}", self.0);
    }
}

fn drop_demo() {
    println!("--- Drop (RAII) ---");
    let _a = Ressource("A");
    {
        let _b = Ressource("B");
        println!("  fin du bloc interne");
    } // B libéré ici
    let c = Ressource("C");
    drop(c); // libération anticipée (`c.drop()` est interdit)
    println!("  fin de la fonction");
    // Ordre : variables libérées dans l'ordre INVERSE de leur déclaration → A en dernier.
}

// ───────────────────────── Rc<T> ─────────────────────────

fn rc_demo() {
    println!("--- Rc : propriété partagée (mono-thread) ---");
    let donnees = Rc::new(String::from("partagé"));
    println!("compteur après création : {}", Rc::strong_count(&donnees));
    let a = Rc::clone(&donnees); // ne copie PAS la String : incrémente le compteur
    {
        let b = Rc::clone(&donnees);
        println!(
            "compteur avec a et b : {} ({a}, {b})",
            Rc::strong_count(&donnees)
        );
    }
    println!("après la sortie de b : {}", Rc::strong_count(&donnees));
    // Rc<T> est en lecture seule. Pour muter : Rc<RefCell<T>> (mono-thread) ou Arc<Mutex<T>> (multi-threads).
}

// ───────────────────────── RefCell<T> ─────────────────────────

/// Mutabilité intérieure : les règles d'emprunt sont vérifiées à l'EXÉCUTION.
/// Violation → `panic!` (ou `Err` avec `try_borrow_mut`).
trait Notificateur {
    fn envoyer(&self, message: &str);
}

struct FauxNotificateur {
    messages: RefCell<Vec<String>>, // mutable à travers &self
}

impl Notificateur for FauxNotificateur {
    fn envoyer(&self, message: &str) {
        self.messages.borrow_mut().push(message.to_string());
    }
}

fn refcell_demo() {
    println!("--- RefCell / Cell ---");
    let n = FauxNotificateur {
        messages: RefCell::new(vec![]),
    };
    n.envoyer("un");
    n.envoyer("deux");
    println!("{:?}", n.messages.borrow());

    // Violation détectée à l'exécution (sans panic grâce à try_borrow_mut) :
    let cellule = RefCell::new(1);
    let lecture = cellule.borrow();
    println!(
        "emprunt mutable pendant une lecture ? {:?}",
        cellule.try_borrow_mut().is_err()
    );
    drop(lecture);

    // Cell<T> : pour les types Copy, get/set sans emprunt du tout.
    let compteur = Cell::new(0);
    compteur.set(compteur.get() + 1);
    println!("Cell = {}", compteur.get());

    // Combinaison classique : Rc<RefCell<T>> = valeur partagée ET mutable.
    let partage = Rc::new(RefCell::new(vec![1, 2]));
    let autre = Rc::clone(&partage);
    autre.borrow_mut().push(3);
    println!("{:?}", partage.borrow());
}

// ───────────────────────── Weak<T> : éviter les cycles ─────────────────────────

/// Un arbre où les enfants pointent vers leur parent : un `Rc` des deux côtés créerait un
/// cycle (fuite mémoire). Le parent est donc un `Weak` (ne compte pas pour la libération).
#[derive(Debug)]
struct Noeud {
    valeur: i32,
    parent: RefCell<Weak<Noeud>>,
    enfants: RefCell<Vec<Rc<Noeud>>>,
}

fn arbre_demo() {
    println!("--- Weak : arbre parent/enfants ---");
    let feuille = Rc::new(Noeud {
        valeur: 3,
        parent: RefCell::new(Weak::new()),
        enfants: RefCell::new(vec![]),
    });
    println!(
        "parent de la feuille = {:?}",
        feuille.parent.borrow().upgrade().map(|p| p.valeur)
    );

    let branche = Rc::new(Noeud {
        valeur: 5,
        parent: RefCell::new(Weak::new()),
        enfants: RefCell::new(vec![Rc::clone(&feuille)]),
    });
    *feuille.parent.borrow_mut() = Rc::downgrade(&branche);

    println!(
        "parent de la feuille = {:?}",
        feuille.parent.borrow().upgrade().map(|p| p.valeur)
    );
    println!(
        "branche : strong={} weak={} | feuille : strong={} weak={}",
        Rc::strong_count(&branche),
        Rc::weak_count(&branche),
        Rc::strong_count(&feuille),
        Rc::weak_count(&feuille)
    );
    println!("enfants de la branche : {}", branche.enfants.borrow().len());
}
