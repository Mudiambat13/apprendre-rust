//! # Chapitre 18 — `unsafe` Rust
//!
//! `unsafe` ne désactive PAS le borrow checker. Il débloque cinq « super-pouvoirs » :
//!   1. déréférencer un pointeur brut (`*const T`, `*mut T`) ;
//!   2. appeler une fonction `unsafe` (dont les fonctions FFI) ;
//!   3. accéder à/modifier une `static mut` (à éviter : préférer atomiques / `Mutex`) ;
//!   4. implémenter un trait `unsafe` (`Send`, `Sync`, …) ;
//!   5. accéder aux champs d'une `union`.
//!
//! Principe : on encapsule l'`unsafe` dans une API **sûre** (*sound*), en documentant les
//! invariants (`// SAFETY: …`). Un `unsafe` mal justifié est un bug de l'auteur, pas du langage.
//!
//! Lancer : `cargo run --bin ch18_unsafe`
//! Vérifier : `cargo +nightly miri test --bin ch18_unsafe`  (détecte les comportements indéfinis)

use std::alloc::{self, Layout};
use std::ffi::{CString, c_char};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

// ───────────────────────── 1. Pointeurs bruts ─────────────────────────

fn pointeurs_bruts() {
    println!("--- Pointeurs bruts ---");
    let mut n = 10;
    // Créer un pointeur brut est SÛR ; c'est le déréférencement qui ne l'est pas.
    let p_lecture = &n as *const i32;
    let p_ecriture = &mut n as *mut i32;
    unsafe {
        // SAFETY: les deux pointeurs viennent de références valides vers `n`, vivante ici ;
        // on n'a pas d'autre accès concurrent.
        *p_ecriture += 5;
        println!("*p_lecture = {}", *p_lecture);
    }

    // Arithmétique de pointeurs sur un tableau.
    let t = [10, 20, 30, 40];
    let debut = t.as_ptr();
    let troisieme = unsafe {
        // SAFETY: l'indice 2 est dans les bornes du tableau de 4 éléments.
        *debut.add(2)
    };
    println!("t[2] via pointeur = {troisieme}");
}

// ───────────────────────── 2. API sûre au-dessus d'unsafe ─────────────────────────

/// Réimplémentation de `slice::split_at_mut` : deux `&mut` sur des zones DISJOINTES.
/// Le borrow checker ne sait pas le prouver, nous si.
pub fn diviser_en_deux<T>(tranche: &mut [T], milieu: usize) -> (&mut [T], &mut [T]) {
    let len = tranche.len();
    assert!(milieu <= len, "milieu hors limites"); // l'invariant qui rend le code sûr
    let ptr = tranche.as_mut_ptr();
    unsafe {
        // SAFETY: [0, milieu) et [milieu, len) ne se chevauchent pas, restent dans la
        // tranche d'origine, et `tranche` est empruntée en exclusivité pour la durée du retour.
        (
            std::slice::from_raw_parts_mut(ptr, milieu),
            std::slice::from_raw_parts_mut(ptr.add(milieu), len - milieu),
        )
    }
}

// ───────────────────────── 3. FFI : appeler du C ─────────────────────────

// Edition 2024 : un bloc extern doit être `unsafe extern`. On peut marquer `safe` les
// fonctions dont on garantit qu'elles ne peuvent pas causer de comportement indéfini.
unsafe extern "C" {
    safe fn abs(entree: i32) -> i32;
    fn strlen(s: *const c_char) -> usize;
}

fn ffi() {
    println!("--- FFI ---");
    println!("abs(-7) via libc = {}", abs(-7));
    let c_texte = CString::new("bonjour").expect("pas d'octet nul");
    let n = unsafe {
        // SAFETY: `c_texte` est une chaîne C valide, terminée par un octet nul, qui vit
        // pendant tout l'appel.
        strlen(c_texte.as_ptr())
    };
    println!("strlen(\"bonjour\") via libc = {n}");
}

// ───────────────────────── 4. Un Vec<T> écrit à la main ─────────────────────────

/// Version pédagogique de `Vec<T>` : allocation manuelle, croissance, `Drop`, `Deref<[T]>`.
/// (Types de taille zéro non gérés pour rester simple.)
pub struct MonVec<T> {
    ptr: NonNull<T>, // jamais nul ; `dangling()` tant qu'on n'a rien alloué
    cap: usize,
    len: usize,
}

impl<T> MonVec<T> {
    pub fn new() -> Self {
        assert!(
            size_of::<T>() != 0,
            "types de taille zéro non gérés dans cet exemple"
        );
        MonVec {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn grandir(&mut self) {
        let nouvelle_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let nouveau_layout = Layout::array::<T>(nouvelle_cap).expect("capacité trop grande");

        let nouveau_ptr = if self.cap == 0 {
            // SAFETY: le layout est de taille non nulle (T non ZST, nouvelle_cap >= 4).
            unsafe { alloc::alloc(nouveau_layout) }
        } else {
            let ancien_layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: `self.ptr` a été alloué avec `ancien_layout` par le même allocateur.
            unsafe {
                alloc::realloc(
                    self.ptr.as_ptr().cast::<u8>(),
                    ancien_layout,
                    nouveau_layout.size(),
                )
            }
        };

        self.ptr = match NonNull::new(nouveau_ptr.cast::<T>()) {
            Some(p) => p,
            None => alloc::handle_alloc_error(nouveau_layout),
        };
        self.cap = nouvelle_cap;
    }

    pub fn push(&mut self, valeur: T) {
        if self.len == self.cap {
            self.grandir();
        }
        // SAFETY: len < cap après `grandir`, donc l'emplacement est alloué mais non initialisé ;
        // `write` ne lit ni ne détruit l'ancienne valeur (il n'y en a pas).
        unsafe { ptr::write(self.ptr.as_ptr().add(self.len), valeur) };
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // SAFETY: l'élément `len` était initialisé ; on le « sort » et on le considère
            // désormais comme non initialisé (len a été décrémenté avant).
            Some(unsafe { ptr::read(self.ptr.as_ptr().add(self.len)) })
        }
    }
}

impl<T> Default for MonVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for MonVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while self.pop().is_some() {} // détruit chaque élément (appelle leur `Drop`)
            let layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: allocation faite avec ce même layout, plus aucun élément vivant.
            unsafe { alloc::dealloc(self.ptr.as_ptr().cast::<u8>(), layout) };
        }
    }
}

impl<T> Deref for MonVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        // SAFETY: `len` éléments initialisés à partir de `ptr` (même quand `ptr` est
        // `dangling` avec len == 0 : permis pour une tranche vide bien alignée).
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for MonVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        // SAFETY: idem, avec exclusivité garantie par `&mut self`.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

// `NonNull<T>` n'est ni `Send` ni `Sync` : on affirme que notre type se comporte comme `Vec<T>`.
// SAFETY: MonVec possède ses données de manière exclusive, comme Vec<T>.
unsafe impl<T: Send> Send for MonVec<T> {}
unsafe impl<T: Sync> Sync for MonVec<T> {}

fn mon_vec() {
    println!("--- MonVec<T> ---");
    let mut v = MonVec::new();
    for mot in ["un", "deux", "trois", "quatre", "cinq"] {
        v.push(mot.to_string());
    }
    println!("{} éléments : {:?}", v.len(), &*v);
    v.sort(); // méthodes de slice, grâce à Deref<Target=[T]>
    println!("trié : {:?}", &v[..]);
    println!("pop → {:?}", v.pop());
}

fn main() {
    pointeurs_bruts();
    let mut tab = [1, 2, 3, 4, 5];
    let (a, b) = diviser_en_deux(&mut tab, 2);
    a[0] = 100;
    b[0] = 300;
    println!("diviser_en_deux → {tab:?}");
    ffi();
    mon_vec();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn push_pop_et_croissance() {
        let mut v = MonVec::new();
        for i in 0..100 {
            v.push(i);
        }
        assert_eq!(v.len(), 100);
        assert_eq!(v[42], 42);
        assert_eq!(v.pop(), Some(99));
        assert_eq!(v.iter().sum::<i32>(), (0..99).sum());
    }

    #[test]
    fn deref_donne_les_methodes_de_slice() {
        let mut v = MonVec::new();
        v.push(3);
        v.push(1);
        v.push(2);
        v.sort();
        assert_eq!(&*v, &[1, 2, 3]);
    }

    /// Vérifie qu'il n'y a ni fuite ni double destruction (Miri le confirme aussi).
    #[test]
    fn chaque_element_est_detruit_une_fois() {
        struct Compteur(Rc<Cell<u32>>);
        impl Drop for Compteur {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        let drops = Rc::new(Cell::new(0));
        {
            let mut v = MonVec::new();
            for _ in 0..10 {
                v.push(Compteur(Rc::clone(&drops)));
            }
            drop(v.pop()); // 1 destruction immédiate
        } // 9 restantes détruites ici
        assert_eq!(drops.get(), 10);
    }

    #[test]
    fn diviser_en_deux_disjoint() {
        let mut t = [1, 2, 3, 4];
        let (a, b) = diviser_en_deux(&mut t, 1);
        std::mem::swap(&mut a[0], &mut b[2]);
        assert_eq!(t, [4, 2, 3, 1]);
    }

    #[test]
    #[should_panic(expected = "hors limites")]
    fn diviser_en_deux_hors_limites() {
        let mut t = [1, 2];
        let _ = diviser_en_deux(&mut t, 3);
    }
}
