# ⚠️ Pièges courants (et comment s'en sortir)

Format : **symptôme → cause → remède**.

## Ownership et emprunts

| Message du compilateur | Cause | Remède |
|:--|:--|:--|
| `borrow of moved value` | la valeur a été déplacée (affectation, appel de fonction, `for x in v`) | emprunter (`&v`), `clone()` si nécessaire, ou itérer avec `.iter()` |
| `cannot borrow as mutable because it is also borrowed as immutable` | une référence en lecture est encore utilisée | réduire sa portée, copier la valeur utile avant la mutation |
| `cannot borrow *x as mutable more than once` | deux `&mut` simultanés | restructurer ; `split_at_mut`, `mem::take`, ou découper la struct en champs |
| `borrowed value does not live long enough` | la référence survit à sa source | retourner une valeur possédée, étendre la portée de la source |
| `cannot move out of borrowed content / index` | on déplace un élément hors d'un `&`/`Vec` | `.clone()`, `.remove(i)`, `mem::take`, `Option::take` |
| `missing lifetime specifier` | plusieurs références en entrée, sortie ambiguë | annoter `'a` (chapitre 8) |

## Types

| Symptôme | Remède |
|:--|:--|
| `expected &str, found String` | écrire `&s` (ou `s.as_str()`) |
| `expected String, found &str` | `.to_string()` / `.into()` |
| `mismatched types: expected i32, found usize` | conversion explicite : `x as usize`, `i32::try_from(x)?` |
| `expected (), found i32` en fin de fonction | supprimer le `;` final |
| `the size of … cannot be known at compilation time` | `Box<dyn Trait>`, `&dyn Trait` ou un générique |
| `trait bound X: Trait is not satisfied` | ajouter la contrainte (`T: Display`) ou implémenter le trait |
| `type annotations needed` | `collect::<Vec<_>>()`, `parse::<i32>()`, annoter la variable |

## Comportements surprenants à l'exécution

| Piège | Explication |
|:--|:--|
| Débordement entier : panic en debug, **wrap silencieux en release** | utilisez `checked_`, `saturating_`, `wrapping_` selon l'intention |
| `&s[0..1]` panique sur un texte accentué | indices en **octets** ; couper sur une frontière de caractère (`char_indices`) |
| Ordre d'itération d'un `HashMap` change à chaque exécution | volontaire ; triez ou `BTreeMap` |
| `0.1 + 0.2 != 0.3` | flottants IEEE-754 ; comparer avec une tolérance |
| `v[i]` panique, `v.get(i)` non | préférez `get` pour les entrées non fiables |
| `RefCell` panique en cas d'emprunt double | `try_borrow_mut`, ou revoir la conception |
| Un thread qui panique ne tue pas le programme | vérifier le `Result` de `join()` |
| `Mutex` empoisonné | `lock().unwrap_or_else(|e| e.into_inner())` si la donnée reste exploitable |
| Boucle `for` qui ne finit pas avec un canal `mpsc` | oubli de `drop(tx)` : le récepteur attend toujours |
| `let _ = x;` détruit **tout de suite** ; `let _x = x;` garde jusqu'à la fin du bloc | utile pour les gardes (verrou) |

## Async

| Piège | Remède |
|:--|:--|
| Le futur « ne fait rien » | un futur est paresseux : `.await` ou `spawn` |
| Tout le serveur ralentit | un appel **bloquant** dans une tâche : `spawn_blocking`, versions async |
| `future cannot be sent between threads safely` | un type non-`Send` (ex. `Rc`, `MutexGuard`) traverse un `.await` : réduire sa portée |
| Verrou `std::sync::Mutex` tenu à travers un `.await` | `tokio::sync::Mutex` ou relâcher avant d'attendre |

## Performance

| Symptôme | Cause probable |
|:--|:--|
| « Rust est plus lent que Python » | build **debug** : utilisez `--release` |
| Beaucoup d'allocations | `format!`/`clone()`/`collect()` en boucle chaude |
| Lent sur de gros tableaux | parcours colonne par colonne (cache) ; `Vec<Vec<T>>` au lieu d'un tableau plat |

## Hygiène

- Lancez `cargo clippy` **régulièrement** ; il corrige vos habitudes.
- Ne luttez pas contre le compilateur en clonant partout : si le borrow checker résiste, la structure de données est souvent à repenser (indices, identifiants, arène).
- Commitez souvent : c'est plus facile de revenir en arrière quand un refactoring d'ownership dérape.
