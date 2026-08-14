// ============================================================
//  STRUCTS USE-CASE : A tiny library (book records)
// ============================================================
// This puts everything from the `structs` teaching project into a
// small, real program: named structs, methods, derived traits, and
// Rust error handling (Result / Option) all in one.
//
// Build & run:   cargo run --release
// (Run in the structs_usecase/ directory)

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

// A newtype tuple struct wrapping an id so we never mix it up with
// a plain u32.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BookId(u32);

#[derive(Debug)]
struct Library {
    books: HashMap<BookId, Book>,
    next_id: u32,
}

#[derive(Debug, PartialEq)]
enum LibraryError {
    NotFound(BookId),
}

impl Book {
    fn new(title: &str, author: &str, pages: u32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
        }
    }
}

impl Library {
    fn new() -> Library {
        Library {
            books: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_book(&mut self, book: Book) -> BookId {
        let id = BookId(self.next_id);
        self.next_id += 1;
        self.books.insert(id, book);
        id
    }

    // &self -> read only, returns a reference to the book.
    fn get_book(&self, id: BookId) -> Option<&Book> {
        self.books.get(&id)
    }

    fn remove_book(&mut self, id: BookId) -> Result<Book, LibraryError> {
        self.books.remove(&id).ok_or(LibraryError::NotFound(id))
    }

    fn list(&self) {
        if self.books.is_empty() {
            println!("  (library is empty)");
            return;
        }
        for (id, book) in &self.books {
            println!("  [{}] {} by {} ({} pages)", id.0, book.title, book.author, book.pages);
        }
    }
}

fn main() {
    let mut library = Library::new();

    // Seed the library with a few books.
    let rust = library.add_book(Book::new("Rust in Action", "Tim McNamara", 496));
    let learning = library.add_book(Book::new("The Rust Programming Language", "Steve Klabnik", 528));
    let _algo = library.add_book(Book::new("CLRS", "Cormen et al.", 1312));

    println!("=== library catalogue ===");
    library.list();

    // Look up a book by id - returns Option.
    println!("\n=== lookups ===");
    match library.get_book(rust) {
        Some(book) => println!("  found: {} by {}", book.title, book.author),
        None => println!("  no book with that id"),
    }

    // Borrowing + returning with Result-based error handling.
    let borrower = String::from("Alice");

    println!("\n=== removing / error handling ===");
    match library.remove_book(learning) {
        Ok(book) => println!("  {borrower} borrowed: {} by {}", book.title, book.author),
        Err(e) => println!("  ERROR: {e:?}"),
    }

    // Try to remove the same book again -> it is gone -> NotFound error.
    match library.remove_book(learning) {
        Ok(book) => println!("  removed again: {book:?}"),
        Err(e) => println!("  ERROR: {e:?} (expected - already removed)"),
    }

    // Try a book id that never existed.
    match library.remove_book(BookId(999)) {
        Ok(_) => println!("  removed"),
        Err(e) => println!("  ERROR: {e:?} (expected - never existed)"),
    }

    println!("\n=== final catalogue ===");
    library.list();

    // Compare two books - PartialEq.
    let a = Book::new("X", "Y", 100);
    let b = a.clone();
    println!("\nclone equality: a == b ? {}", a == b);
}