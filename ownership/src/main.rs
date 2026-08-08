// ============================================================
//  RUST OWNERSHIP - RULES, EXAMPLES, ERRORS, LEARNING
// ============================================================
//
// Memory management in Rust has NO garbage collector and NO manual
// free() - instead it uses OWNERSHIP, checked entirely at compile time.
//
// THE THREE RULES OF OWNERSHIP
//   1. Every value has exactly ONE owner at any time.
//   2. When the owner goes out of scope, the value is freed (dropped).
//   3. Transferring ownership (a "move") makes the old owner unusable.
//
// Follow the sections in order. The section 5 comments contain code
// that DOES NOT COMPILE - uncomment them one by one and run
// `cargo check` to see the real compiler errors. That is the best
// way to learn.

// ============================================================
//  SECTION 1 : WHAT OWNERSHIP LOOKS LIKE
// ============================================================
fn section1() {
    println!("=== 1. EVERY VALUE HAS ONE OWNER ===");

    // `owner` is the owner of the heap string.
    // When main's scope ends, the memory is freed automatically.
    let owner = String::from("I live on the heap");
    println!("owner: {owner} (freed automatically at scope end)");

    // Copy types are the exception: they live on the stack entirely.
    let x: i32 = 5;
    let y: i32 = x; // Copy - x is DUPLICATED, both stay valid
    println!("Copy: x={x} y={y}  (both alive)");

    // Heap types are moved, not copied.
    let s1 = String::from("hello");
    let s2 = s1; // s1's ownership MOVES to s2
    println!("Move: s2={s2}");
    // println!("{s1}"); // COMPILE ERROR -> s1 already moved (see section 5)
}

// ============================================================
//  SECTION 2 : WHY OWNERSHIP? (THE PROBLEMS IT SOLVES)
// ============================================================
fn section2() {
    println!("=== 2. WHY USE OWNERSHIP AT ALL? ===");
    println!("Without ownership, languages suffer these classic bugs:");
    println!("  - Use-after-free      : read memory that was already freed");
    println!("  - Double-free         : free the same memory twice, crash");
    println!("  - Memory leak         : memory never freed, app grows forever");
    println!("  - Dangling pointer    : pointer still alive but data gone");
    println!();
    println!("How others handle it (and the cost we avoid):");
    println!("  - C/C++               : manual free() -> all the bugs above");
    println!("  - Java/Go (GC)        : automatic but pauses + CPU overhead");
    println!("  - Rust                : OWNERSHIP, solved at compile time");
    println!();
    println!("Benefits:");
    println!("  1. Zero runtime cost - no GC thread, no pause.");
    println!("  2. No manual memory code - the compiler owns the homework.");
    println!("  3. Bugs caught at compile time, never in production.");
    println!("  4. Safe by default - memory bugs usually become RCE bugs.");
}

// ============================================================
//  SECTION 3 : OWNERSHIP IN FUNCTIONS
// ============================================================
fn take_ownership(value: String) {
    // value now owns the string
    println!("take_ownership got: {}", value);
    // value is freed (dropped) here - end of function = end of owner
}

fn return_ownership() -> String {
    // Creating inside, transferring ownership to the caller:
    String::from("built inside, returned to caller")
}

// ============================================================
//  SECTION 4 : BORROWING - USING WITHOUT OWNING
// ============================================================
fn print_borrowed(text: &str) {
    println!("      borrowed (immutable): {text}");
}

fn add_suffix(text: &mut String, suffix: &str) {
    // Receiving a &mut reference is NOT a move - the caller
    // keeps ownership of the string; add_suffix just borrows.
    text.push_str(suffix);
}

fn section4() {
    let mut my_string = String::from("hello");
    println!("before: {my_string}");

    // The function below takes &String but &String coerces to &str.
    print_borrowed(&my_string); // borrow, my_string keeps ownership

    add_suffix(&mut my_string, " world");
    println!("after mutable borrow: {my_string}");

    // Borrow RULES (checked at compile time):
    //   Many immutable borrows (&)  -> OK, parallel readers.
    //   One mutable borrow (&mut)   -> only one at a time.
    //   No mixing & and &mut for the same variable in scope.
}

// ============================================================
//  SECTION 5 : ERRORS - CODE THAT MUST NOT COMPILE (the lesson)
// ============================================================
// This section's code is commented ON PURPOSE. Uncomment any one
// of them to see a real compiler error, run `cargo check`, fix it,
// then uncomment the next.
// DO NOT uncomment all - the project may stop compiling (which is
// actually the goal of this exercise).

// --- ERROR 1: Use of moved value (rule 3) --------------------
// let s1 = String::from("x");
// let s2 = s1;                     -> moves s1
// println!("{}", s1);              -> E0382: borrow of moved value

// --- ERROR 2: Move is one-way; use it after 2nd move --------
// let s = String::from("s");
// take_ownership(s);               -> move #1
// take_ownership(s);               -> E0382: use after move

// --- ERROR 3: Double mutable borrows ------------------------
// let mut word = String::from("w");
// let m1 = &mut word;
// let m2 = &mut word;              -> E0499: cannot borrow as mutable
// println!("{} {}", m1, m2);          more than once at a time

// --- ERROR 4: Immutable + mutable mix ------------------------
// let mut n = String::from("n");
// let im = &n;
// let mm = &mut n;                 -> E0499: cannot borrow as mutable
// println!("{} {}", im, mm);          because also borrowed immutable

// --- ERROR 5: Dangling reference --------------------------------
// fn dangle() -> &String {
//     let inside = String::from("tmp");
//     &inside                      -> E0106: missing lifetime
// }                                -> returning reference to local variable
// index requires owner to live longer

// --- ERROR 6: Many: single mutable + immutable -----------------
// let mut v = String::from("v");
// let mv = &mut v;
// println!("{}", v);               -> E0502: cannot borrow v as immutable
//     but NOT the goal of that one,
//     is borrow of an immutable while mutable borrow exists.

// ============================================================
//  SECTION 6 : SLICING - BORROWED PART OF AN ARRAY
// ============================================================
fn section6() {
    let words = ["one", "two", "three", "four"];

    // &words[1..3] borrows a slice WITHOUT sending ownership
    let middle = &words[1..3];
    println!("sliced: {:?} (borrowed from {:?})", middle, words);
}

// ============================================================
//  SECTION 7 : LEARNING - RECAP AT THE END
// ============================================================
fn section7() {
    println!("\n=============== LEARNING (recap) ===============");
    println!("1. Every value has exactly ONE owner.");
    println!("2. Stack types (i32, f64, bool, char, &str) are Copy: cheap to duplicate.");
    println!("3. Heap types (String, Vec, Box) MOVE - old variable becomes invalid.");
    println!("4. A function can get a value back only if it RETURNS it.");
    println!("5. Borrowing never transfers ownership: & = read-only, &mut = write.");
    println!("6. Borrow rules: many &, one &mut, never & + &mut at same time.");
    println!("7. Ownership exists so the compiler can free memory safely.");
    println!("8. Result: no GC, no manual free, no UAF / double-free / leaks.");
    println!("9. Untreated moved value is a compile error, not a crash.");
    println!("10. The borrow checker is your friend - it GUARANTEES safety.");
}

fn main() {
    section1();
    section2();
    println!("=== Take ownership ===");
    take_ownership(String::from("moved into take_ownership"));
    // after this line the String is freed. Not usable here:
    //     let s = String::from("again");
    //     take_ownership(s);
    //     println!("{s}");   // <======== compile error
    let back = return_ownership();
    println!("returned from function: {back}");

    section4();
    section6();
    section7();
}