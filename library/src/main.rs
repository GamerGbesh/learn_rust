mod library;
mod book;
mod member;

use library::Library;
use book::{Book, BookStatus};
use member::{Member, MemberType};
use chrono::Utc;

fn main() {
    println!("=== Library Management System ===\n");

    // Create a new library
    let mut lib = Library::new(
        String::from("City Central Library"),
        String::from("123 Main Street"),
        String::from("+1-234-567-8900"),
        String::from("info@citylibrary.com")
    );

    // Initialize categories
    lib.initialize_categories();
    println!("Library initialized with standard categories\n");

    // Add some books
    let book1 = Book::new(
        String::from("The Rust Programming Language"),
        String::from("978-1-59327-828-1"),
        String::from("Steve Klabnik and Carol Nichols"),
        String::from("Technical"),
        String::from("Tech"),
        1,
        String::from("No Starch Press"),
        Utc::now() - chrono::Duration::days(365),
        String::from("A comprehensive guide to Rust programming"),
        552,
        String::from("English")
    );

    let book2 = Book::new(
        String::from("Clean Code"),
        String::from("978-0-13-235088-4"),
        String::from("Robert C. Martin"),
        String::from("Software Engineering"),
        String::from("Tech"),
        1,
        String::from("Prentice Hall"),
        Utc::now() - chrono::Duration::days(730),
        String::from("A handbook of agile software craftsmanship"),
        464,
        String::from("English")
    );

    let book3 = Book::new(
        String::from("1984"),
        String::from("978-0-452-28423-4"),
        String::from("George Orwell"),
        String::from("Dystopian"),
        String::from("Fiction"),
        1,
        String::from("Signet Classic"),
        Utc::now() - chrono::Duration::days(3650),
        String::from("A dystopian social science fiction novel"),
        328,
        String::from("English")
    );

    lib.add_book(book1);
    lib.add_book(book2);
    lib.add_book(book3);
    
    println!("Added 3 books to the library");
    lib.view_books();
    println!();

    // Create members
    let mut alice = Member::new(
        String::from("Alice Johnson"),
        25,
        String::from("+1-555-0101"),
        String::from("alice@email.com"),
        MemberType::STUDENT,
        String::from("MEM001")
    );

    let mut bob = Member::new(
        String::from("Bob Smith"),
        35,
        String::from("+1-555-0102"),
        String::from("bob@email.com"),
        MemberType::PREMIUM,
        String::from("MEM002")
    );

    println!("\n=== Member Operations ===");
    println!("Created members: {} and {}\n", alice.name, bob.name);

    // Get mutable references to books from library
    // For demonstration, we'll work with books directly
    // In a real system, you'd want to handle this through the library
    
    // Simulate borrowing books
    println!("=== Borrowing Books ===");
    
    // Re-create book references for borrowing operations
    let mut rust_book = Book::new(
        String::from("The Rust Programming Language"),
        String::from("978-1-59327-828-1"),
        String::from("Steve Klabnik and Carol Nichols"),
        String::from("Technical"),
        String::from("Tech"),
        1,
        String::from("No Starch Press"),
        Utc::now() - chrono::Duration::days(365),
        String::from("A comprehensive guide to Rust programming"),
        552,
        String::from("English")
    );

    let mut clean_code_book = Book::new(
        String::from("Clean Code"),
        String::from("978-0-13-235088-4"),
        String::from("Robert C. Martin"),
        String::from("Software Engineering"),
        String::from("Tech"),
        1,
        String::from("Prentice Hall"),
        Utc::now() - chrono::Duration::days(730),
        String::from("A handbook of agile software craftsmanship"),
        464,
        String::from("English")
    );

    // Alice borrows a book
    if alice.add_borrowed_books(&mut rust_book) {
        println!("✓ {} successfully borrowed '{}'", alice.name, rust_book.name);
    }

    // Bob borrows a book
    if bob.add_borrowed_books(&mut clean_code_book) {
        println!("✓ {} successfully borrowed '{}'", bob.name, clean_code_book.name);
    }

    println!("\n=== Member's Borrowed Books ===");
    print!("{}'s books: ", alice.name);
    alice.show_books();
    print!("{}'s books: ", bob.name);
    bob.show_books();

    // Try to borrow an already borrowed book
    println!("\n=== Testing Book Already Borrowed ===");
    alice.add_borrowed_books(&mut rust_book);

    // Return a book
    println!("\n=== Returning Books ===");
    if alice.remove_borrowed_book(&mut rust_book) {
        println!("Book status after return: {:?}", match rust_book.status {
            BookStatus::AVAILABLE => "Available",
            BookStatus::BORROWED => "Borrowed"
        });
    }

    // Test reservation system
    println!("\n=== Book Reservation ===");
    alice.reserve_book(&clean_code_book);
    println!("Alice has {} reserved book(s)", alice.reserved_books.len());

    // Cancel reservation
    alice.cancel_reservation(&clean_code_book);
    println!("After cancellation, Alice has {} reserved book(s)", alice.reserved_books.len());

    // Test debt payment
    println!("\n=== Debt Management ===");
    println!("{}'s current debt: ${:.2}", alice.name, alice.debt);
    
    if alice.debt > 0.0 {
        alice.pay_debt(alice.debt);
        println!("After payment, {}'s debt: ${:.2}", alice.name, alice.debt);
    }

    println!("\n=== Library System Demo Complete ===");
}