fn main() {
    // =================== SCALAR TYPES ===================

    // ---- Integers ----
    // signed: i8 i16 i32 i64 i128 isize | unsigned: u8 u16 u32 u64 u128 usize
    let a: i32 = 10;                  // most common, default int
    let b: u8 = 255;                  // max for u8
    let small: i8 = 100;
    let big: i64 = 9_000_000_000;     // underscores make big numbers readable

    // Unique point: numbers that never go negative should use unsigned types.
    // overflow in debug mode panics at runtime.

    // ---- Floats ----
    let x: f64 = 2.0;                 // f64 default, most precise
    let y: f32 = 3.0;                 // less precise
    let pi: f64 = std::f64::consts::PI;

    // Unique point: f32/f64 never safely compare for exact equality.

    // ---- Boolean ----
    let t: bool = true;
    let f: bool = false;

    // ---- Char ----
    let c: char = 'A';                 // single quotes, single character
    let emoji: char = '\u{1F600}';     // char is 4 bytes, can hold unicode

    // Unique point: char is NOT a byte (8 bits). To store text use &str / String.

    // =================== COMPOUND TYPES ===================

    // ---- Tuple (fixed length, different types allowed) ----
    let tuple = (500, 6.4, "hello");
    let (one, two, three) = tuple; // destructuring
    let first = tuple.0;           // access by index

    // Unique point: tuples are fixed-length; you cannot add/remove elements.

    // ---- Array (fixed length, SAME type) ----
    let arr: [i32; 4] = [1, 2, 3, 4];
    let same: [i32; 100] = [0; 100];  // all 100 elements = 0
    let first_two = arr[1];           // index access

    // Unique point: arrays are fixed-size. Use Vec<T> for dynamic size.

    // ==================== STRING TYPES ===================

    // ---- &str (string slice, immutable, borrowed) ----
    let s1: &str = "hello";
    // Unique point: &str cannot be modified. Strings live in memory as a view.

    // ---- String (owned, growable, heap-allocated) ----
    let mut s2 = String::from("hello");
    s2.push_str(", world");
    s2.push('!');

    // Unique point: String owns its data and CAN be changed.

    // ==================== OWNERSHIP / REFERENCE ============

    let original = String::from("mine");
    let owned_clone = original.clone(); // deep copy
    // move: let moved = original; // original unusable after move -> error if used

    // ---- Reference & ----
    let borrowed: &String = &original;  // immutable borrow
    // Unique point: passing &T does NOT take ownership (borrowing).

    // ---- Mutating reference ----
    let mut owned = String::from("x");
    let mut_ref: &mut String = &mut owned;

    // ==================== MUTABILITY ======================
    // Unique point: variables in Rust are IMMUTABLE by default.
    let mut mutable_val = 1;           // "mut" keyword makes it changeable
    mutable_val += 1;

    // The following lines are just print calls so the compiler doesn't
    // complain about unused variables. Feel free to remove.
    println!("{a} {b} {small} {big} {x} {y} {pi} {t} {f} {c} {emoji}");
    println!("tuple: {} {} {} (first via index: {})", one, two, three, first);
    println!("first: {first_two}, arr[0]: {}", arr[0]);
    println!("same[99] = {}, s1: {}, s2: {}", same[99], s1, s2);
    println!("original: {original}, clone: {owned_clone}");
    println!("borrowed: {borrowed}");
    println!("mutable_val is now: {mutable_val}");
    println!("mut_ref points at: {}", *mut_ref);

    // ==================== COMMON ERRORS YOU WILL GET ====================

    // 1. use of moved value -> E0382
    //    let moved = original;
    //    println!("{}", original); // ERROR: value used after move
    //    Fix: use original.clone() if you need both copies.

    // 2. type mismatch -> E0308
    //    let n: u32 = -5; // ERROR: negative cannot fit into u32
    //    Fix: use i32 (or wider) for signed values.

    // 3. assigning to immutable variable -> E0384
    //    let z = 5;
    //    z = 10; // ERROR: cannot assign twice to immutable variable
    //    Fix: declare `let mut z = 5;`

    // 4. index out of bounds -> panics at RUNTIME (not compile time)
    //    let arr: [i32; 4] = [1, 2, 3, 4];
    //    arr[4]; // panic: index out of bounds, the index is 4, len is 4
    //    Fix: check the length first: if i < arr.len() { arr[i] }

    // 5. integer overflow -> panics in debug builds
    //    let v: u8 = 255;
    //    v + 1; // panic: attempt to add with overflow
    //    Fix: use a bigger type (u16) or wrapping_add(1).

    // 6. cannot borrow `x` as mutable more than once -> E0499
    //    let mut s = String::from("hi");
    //    let r1 = &mut s;
    //    let r2 = &mut s; // ERROR: cannot borrow `s` as mutable more than once
    //    Fix: use one &mut at a time, or give r1 its own scope block.

    // 7. expected `f64`, found `i32` -> E0308 (int vs float)
    //    let z: f64 = 3; // ERROR: expected f64, found integer
    //    Fix: write 3.0 or 3_f64.

    // 8. unclosed quote / missing semicolon -> parser error
    //    let s = "abc; // ERROR: unterminated string literal
    //    Fix: close the string and end statements with `;`

    // 9. mismatched tuple/array length
    //    let tup = (1, 2, 3);
    //    let (a, b) = tup; // ERROR: pattern has 2 fields, tuple has 3
    //    Fix: match the number of bindings to the tuple length.
}