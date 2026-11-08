# 🧾 Aide-mémoire

## Cargo

```bash
cargo new nom [--lib]          cargo add serde        cargo remove serde
cargo run [--release] [--bin x] [-p paquet] [-- args]
cargo build | check | test | bench | doc --open | clean
cargo fmt --all                cargo clippy --all-targets -- -D warnings
cargo tree                     cargo update           cargo publish
rustup update                  rustup component add miri rust-src
```

## Types usuels

| Je veux… | Type |
|:--|:--|
| texte possédé / emprunté | `String` / `&str` |
| suite dynamique / vue | `Vec<T>` / `&[T]` |
| tableau de taille fixe | `[T; N]` |
| valeur facultative | `Option<T>` |
| succès ou erreur | `Result<T, E>` |
| dictionnaire | `HashMap<K, V>` (`BTreeMap` si trié) |
| valeur sur le tas | `Box<T>` |
| plusieurs propriétaires | `Rc<T>` / `Arc<T>` |
| mutation à travers `&` | `RefCell<T>` / `Mutex<T>` / `Atomic*` |
| objet de type inconnu | `Box<dyn Trait>` |

## Conversions

| De → vers | Écriture |
|:--|:--|
| `&str` → `String` | `s.to_string()`, `String::from(s)`, `s.into()` |
| `String` → `&str` | `&s`, `s.as_str()` |
| `&[T]` → `Vec<T>` | `s.to_vec()` |
| nombre → texte | `n.to_string()`, `format!("{n}")` |
| texte → nombre | `s.parse::<i32>()` (renvoie un `Result`) |
| itérateur → collection | `.collect::<Vec<_>>()` |
| `Option<T>` → `Result<T, E>` | `.ok_or(erreur)` |
| `Result<T, E>` → `Option<T>` | `.ok()` |
| type étroit → large | `u64::from(x)` ; type large → étroit : `u8::try_from(x)?` |

## Formatage

```rust
println!("{}", x);        // Display          println!("{x}");  // capture directe
println!("{:?}", x);      // Debug            println!("{:#?}", x);   // Debug joli
println!("{:>8.2}", pi);  // aligné à droite, 2 décimales
println!("{:08b} {:x} {:#X}", n, n, n);       // binaire, hexa
println!("{:<10}|{:^10}|{:>10}", a, b, c);    // gauche / centre / droite
eprintln!("erreur");      // stderr
```

## Patterns courants

```rust
if let Some(x) = opt { … }                 let Some(x) = opt else { return };
while let Some(x) = pile.pop() { … }       match v { 1..=5 => …, n if n > 9 => …, _ => … }
let (a, b) = tuple;                        let Point { x, y } = p;
opt.map(f).unwrap_or(0)                    res.map_err(MonErreur::from)?
```

## Itérateurs

```rust
v.iter().map(f).filter(p).collect::<Vec<_>>()      v.iter().sum::<i32>()
v.iter().enumerate()   .zip(w)   .rev()   .skip(n).take(m)   .windows(2)   .chunks(3)
v.iter().any(p)   .all(p)   .find(p)   .position(p)   .max_by_key(f)   .fold(init, f)
```

## Traits à dériver

`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]`
(`Copy` exige `Clone` ; `Eq` exige `PartialEq` ; `Ord` exige `Eq` + `PartialOrd`).

## Choix rapide

| Situation | Choix |
|:--|:--|
| Argument en lecture seule | `&T` (`&str`, `&[T]`) |
| Argument à modifier | `&mut T` |
| Argument consommé / stocké | `T` |
| Retourner un nouveau texte | `String` |
| Partager entre threads | `Arc<T>` (+ `Mutex` si mutation) |
| Erreur dans une lib / une app | `enum` + `thiserror` / `anyhow` |
| Plusieurs types derrière un trait | `Vec<Box<dyn Trait>>` |
| Performance critique | génériques, `--release`, mesurer |
