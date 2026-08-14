# Rust Structs — Explained with Errors

A companion doc to `src/main.rs`. It focuses on the *common compiler
errors* people hit when learning structs, and why each one happens.

---

## 1. What a struct is

A struct bundles related values into one named type.

```
struct User {
    name:   String,   // field : type
    age:    u8,
    active: bool,
}
```

| Kind          | Syntax                        | When to use                       |
|---------------|-------------------------------|-----------------------------------|
| Named fields  | `struct U { a: i32, b: i32 }` | most of the time                  |
| Tuple struct  | `struct P(i32, i32);`         | small "newtype" wrappers          |
| Unit struct   | `struct Marker;`              | a type with no data (marker)      |

---

## 2. The error "missing field" (E0063)

If you create a struct and leave out a field, the compiler refuses.

```rust
let u = User { name: String::from("x"), age: 3 };
//            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// error[E0063]: missing field `active` in initializer
```

**Why:** every field must have a value — no half-built structs.
**Fix:** add every field, or use `..other` to pull the rest from
another instance:

```rust
let u = User { name: String::from("x"), age: 3, ..base };
```

---

## 3. The error "no field named ..." (E0560)

Spelling a field that does not exist:

```rust
let u = User { name: ..., age: 3, active: true, email: String::new() };
//                                                ^^^^^^^^^^^^^^^^^^^
// error[E0560]: struct `User` has no field named `email`
```

**Why:** Rust refuses to silently ignore unknown fields — a typo
becomes a compile error, not a bug.
**Fix:** remove it, or add the field to the struct.

---

## 4. Printing a struct needs `Debug` (E0277)

`println!("{user}")` on a plain struct fails:

```
error[E0277]: `User` doesn't implement `Debug`
```

**Why:** the struct does not know how to render itself to text.
**Fix:** derive it:

```rust
#[derive(Debug)]
struct User { /* ... */ }
// then: println!("{user:?}");
```

---

## 5. The "partial move" trap (E0382)

Structs with heap fields (`String`, `Vec`) are **not** `Copy`. Moving
one field out invalidates the whole struct:

```rust
let u = User::new("Fay".into(), 40);
let name = u.name;      // moves u.name out of u
println!("{u:?}");      // error[E0382]: use of partially moved value: `u`
```

**Why:** `u.name` is a `String` on the heap; moving it to `name` means
the struct can no longer guarantee it is whole.
**Fix:** borrow it instead (`&u.name`), `clone()` it, or consume the
whole struct.

---

## 6. "not declared as mut" (E0596)

Calling a method that needs `&mut self` on a non-`mut` binding:

```rust
let u = User::new(...);
u.deactivate();   // error[E0596]: cannot borrow `u` as mutable
```

**Fix:** `let mut u = ...;`

---

## 7. Comparing with `==` needs `PartialEq` (E0369)

```rust
if a == b { }   // error[E0369]: binary operation `==` cannot be applied
```

**Fix:** `#[derive(PartialEq)]` (add `Debug` too for printing).

---

## 8. Structs move, they don't copy (E0382)

```rust
struct Pair(u32, u32);
let p = Pair(1, 2);
let q = p;          // MOVES p
println!("{}", p.0); // error[E0382]: use of moved value: `p`
```

**Why:** even stack-only structs are not `Copy` unless you opt in.
**Fix:** derive `Copy` + `Clone`:

```rust
#[derive(Copy, Clone)]
struct Pair(u32, u32);
```

---

## 9. Quick error cheat-sheet

| Code     | Message (short)                 | Fix                              |
|----------|---------------------------------|----------------------------------|
| E0063    | missing field                   | add all fields or `..other`      |
| E0560    | no field named                  | remove / add the field           |
| E0277    | doesn't implement `Debug`       | `#[derive(Debug)]`               |
| E0382    | use of moved / partially moved  | borrow, clone, or consume        |
| E0596    | cannot borrow as mutable        | `let mut`                        |
| E0369    | `==` cannot be applied          | `#[derive(PartialEq)]`           |

**TL;DR** — a struct is data + optional methods. Rust's errors exist to
make sure every struct is fully built (`E0063`/`E0560`), printable when
you print it (`E0277`), comparable when you compare it (`E0369`), and
that you never use it after it moved (`E0382`).