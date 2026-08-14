// ============================================================
//  RUST STRUCTS - TYPES, METHODS, ERRORS, LEARNING
// ============================================================
//
// A struct (short for "structure") is a way to group related data
// together into one named type. Like a "class" in other languages,
// but WITHOUT inheritance - just data + methods on it.
//
// THE 3 KINDS OF STRUCTS
//   1. Named-field structs :  struct User { name: String, age: u8 }
//   2. Tuple structs       :  struct Point(i32, i32);   (fields have
//                              no names, only positions)
//   3. Unit structs        :  struct Marker;           (no fields at
//                              all, used for types/traits)
//
// Follow the sections in order. The section 6 comments contain code
// that DOES NOT COMPILE - uncomment them one by one and run
// `cargo check` to see the real compiler errors. That is the best
// way to learn.

// ============================================================
//  SECTION 1 : DEFINING AND USING A STRUCT
// ============================================================
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    active: bool,
}

fn section1() {
    println!("=== 1. DEFINE, CREATE, READ, UPDATE ===");

    // Fields must all be given when you create a struct.
    // Note: `mut` on the binding allows changing its fields.
    let mut user = User {
        name: String::from("Alice"),
        age: 28,
        active: true,
    };

    // Reading fields with dot notation.
    println!("{} is {} years old (active: {})", user.name, user.age, user.active);

    // Updating a field - only the whole binding needs `mut`.
    user.age = 29;
    println!("Birthday! Now {} is {}", user.name, user.age);

    // `#[derive(Debug)]` lets us print the whole struct at once.
    println!("whole struct: {user:?}");

    // Field order does NOT matter - you can write them in any order.
    let second = User {
        active: false,
        age: 0,
        name: String::from("Bob"),
    };
    println!("second: {second:?}");
}

// ============================================================
//  SECTION 2 : STRUCT UPDATE SYNTAX + USING A FUNCTION
// ============================================================
fn build_user(name: String, age: u8) -> User {
    // Shorthand: when the variable name matches the field name,
    // you can write it once instead of `name: name`.
    User {
        name,
        age,
        active: true,
    }
}

fn section2() {
    println!("\n=== 2. SHORTHAND + UPDATE SYNTAX ===");

    let base = build_user(String::from("Carol"), 22);
    println!("built with shorthand: {base:?}");

    // `..base` copies the REST of the fields from `base`.
    // It must be the LAST entry. This is NOT a reference -
    // the values are copied/moved into the new struct.
    let dave = User {
        name: String::from("Dave"),
        ..base
    };
    println!("dave (age/active copied from base): {dave:?}");

    // NOTE: `base.name` was moved into `dave`, so `base` can no
    // longer be used as a whole:
    // println!("{base:?}");  // COMPILE ERROR -> base.name was moved
}

// ============================================================
//  SECTION 3 : TUPLE STRUCTS AND UNIT STRUCTS
// ============================================================
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Color(u8, u8, u8); // same shape as Point, but a DIFFERENT type

struct Marker; // unit struct: no fields at all

fn section3() {
    println!("\n=== 3. TUPLE STRUCTS + UNIT STRUCTS ===");

    let origin = Point(0, 0, 0);
    let red = Color(255, 0, 0);

    // Fields accessed by position, like tuples.
    println!("origin at ({}, {}, {})", origin.0, origin.1, origin.2);
    println!("red = {red:?} (rgb {}, {}, {})", red.0, red.1, red.2);

    // Point and Color both hold 3 x i32, but they are DIFFERENT types.
    // You cannot pass a Color where a Point is expected. That is the
    // whole point of named types.

    // Unit struct: no data, just a type. Useful as a marker.
    let _m = Marker;
    println!("unit struct has zero size ({} bytes)", std::mem::size_of::<Marker>());
}

// ============================================================
//  SECTION 4 : METHODS (impl blocks)
// ============================================================
impl User {
    // `&self` = read-only method. Does not take ownership.
    fn is_active(&self) -> bool {
        self.active
    }

    // `&mut self` = can change the struct.
    fn deactivate(&mut self) {
        self.active = false;
    }

    // Owns self (moves it). Rare - the struct is consumed.
    fn into_string(self) -> String {
        format!("{} ({})", self.name, self.age)
    }

    // Associated function (no self) - called as User::new(...).
    // Acts like a constructor.
    fn new(name: String, age: u8) -> User {
        User {
            name,
            age,
            active: true,
        }
    }
}

fn section4() {
    println!("\n=== 4. METHODS INSIDE impl BLOCKS ===");

    let mut u = User::new(String::from("Eve"), 31);
    println!("Eve active? {}", u.is_active());

    u.deactivate();
    println!("after deactivate: active? {}", u.is_active());

    // into_string CONSUMES u - after this, u is gone (moved).
    let text = u.into_string();
    println!("consumed: {text}");
    // println!("{u:?}"); // COMPILE ERROR -> u was moved
}

// ============================================================
//  SECTION 5 : DERIVE - FREE TRAITS FOR YOUR STRUCTS
// ============================================================
#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    pages: u32,
}

fn section5() {
    println!("\n=== 5. DERIVED TRAITS (Debug, Clone, PartialEq) ===");

    let a = Book {
        title: String::from("Rust in Action"),
        pages: 496,
    };
    let b = a.clone(); // Clone = deep copy, both stay alive
    let c = Book {
        title: String::from("Rust in Action"),
        pages: 496,
    };

    println!("a: {a:?}"); // Debug = printable
    println!("b is a clone, c is a fresh equal struct");
    println!("a == c ? {}", a == c); // PartialEq = comparable with ==
    println!("a == b ? {}", a == b);
}

// ============================================================
//  SECTION 6 : ERRORS - CODE THAT MUST NOT COMPILE (the lesson)
// ============================================================
// This section's code is commented ON PURPOSE. Uncomment any one
// of them to see a real compiler error, run `cargo check`, fix it,
// then uncomment the next.
// DO NOT uncomment all - the project may stop compiling (which is
// actually the goal of this exercise).

// --- ERROR 1: Printing without Debug ---------------------------------
// println!("{user:?}");       -> error[E0277]: `User` doesn't implement
//                                `Debug` (if you remove #[derive(Debug)])
//                                The fix: add #[derive(Debug)] above it.

// --- ERROR 2: Missing field ------------------------------------------
// let u = User { name: String::from("x"), age: 3 };
//                              -> error[E0063]: missing field `active`
//                                in initializer

// --- ERROR 3: Unknown field ------------------------------------------
// let u = User { name: String::from("x"), age: 3, active: true, email: String::new() };
//                              -> error[E0560]: struct `User` has no
//                                field named `email`

// --- ERROR 4: No implicit Copy (move of a field) ----------------------
// let u = User::new(String::from("Fay"), 40);
// let name = u.name;           -> moves `u.name` out of `u`
// println!("{u:?}");           -> error[E0382]: use of partially moved
//                                value: `u`  (name lives on the heap,
//                                so it MOVES instead of copying)

// --- ERROR 5: Tuple struct wrong count --------------------------------
// let p = Point(1, 2);
//                              -> error[E0063]: missing field `2` /
//                                wrong number of arguments

// --- ERROR 6: Mutable method on non-mut binding -----------------------
// let u = User::new(String::from("Gus"), 10);
// u.deactivate();              -> error[E0596]: cannot borrow `u` as
//                                mutable, as it is not declared as
//                                `let mut`

// --- ERROR 7: `==` without PartialEq ----------------------------------
// let a = Book { ... };
// let b = Book { ... };
// if a == b { }                -> error[E0369]: binary operation `==`
//                                cannot be applied to type `Book`
//                                (unless you derive PartialEq)

// --- ERROR 8: Whole struct is not Copy --------------------------------
// struct Pair(u32, u32);
// let p = Pair(1, 2);
// let q = p;                   -> Pair is NOT Copy, so p is MOVED
// println!("{}", p.0);         -> error[E0382]: use of moved value: `p`
//   (derive Copy+Clone if you want cheap duplication)

// ============================================================
//  SECTION 7 : LEARNING - RECAP AT THE END
// ============================================================
fn section7() {
    println!("\n=============== LEARNING (recap) ===============");
    println!("1. struct = named group of fields. 3 kinds: named, tuple, unit.");
    println!("2. Create with struct literal: User {{ name, age }} (order free).");
    println!("3. Shorthand `name` == `name: name`. `..other` copies remaining fields.");
    println!("4. Dot notation reads and (with `mut`) writes fields.");
    println!("5. Methods live in `impl`: &self, &mut self, self, or no self.");
    println!("6. No self = associated function, called as User::new(...).");
    println!("7. derive(Debug, Clone, PartialEq) adds useful free traits.");
    println!("8. Structs are NOT Copy: passing/moving them transfers ownership.");
    println!("9. Struct types with the same shape are still different types.");
    println!("10. Common errors: E0063 missing field, E0382 partial move,");
    println!("    E0277 missing Debug, E0596 not mut, E0369 no PartialEq.");
}

fn main() {
    section1();
    section2();
    section3();
    section4();
    section5();
    section7();
}