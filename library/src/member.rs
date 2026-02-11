use chrono::{DateTime, Utc};

use crate::book::{Book, BookStatus};

pub struct MemberType{
    max_books: u8,
    max_days: u8,
    fine_rate: f32,
}

impl MemberType{
    pub const STUDENT: MemberType = MemberType { max_books: 3, max_days: 14, fine_rate: 0.50 };
    pub const REGULAR: MemberType = MemberType { max_books: 2, max_days: 7, fine_rate: 1.00 };
    pub const PREMIUM: MemberType = MemberType { max_books: 5, max_days: 21, fine_rate: 0.25 };
}


pub struct Member{
    pub name: String,
    pub age: u8,
    pub contact: String,
    pub email: String,
    pub debt: f32,
    pub member_type: MemberType,
    pub membership_start_date: DateTime<Utc>,
    pub membership_end_date: DateTime<Utc>,
    pub member_id: String,
    pub books_possessed: Vec<String>,
    pub reserved_books: Vec<String>
}


impl Member {
    pub fn new(name: String, age: u8, contact: String, email: String, member_type: MemberType, member_id: String) -> Self {
        Member {
            name,
            age,
            contact,
            email,
            debt: 0.0,
            member_type,
            membership_start_date: Utc::now(),
            membership_end_date: Utc::now() + chrono::Duration::days(365),
            member_id,
            books_possessed: Vec::new(),
            reserved_books: Vec::new()
        }
    }

    pub fn show_books(&self){
        for (counter, book) in self.books_possessed.iter().enumerate() {
            println!("{}. {}", counter, book);
        }
    }

    pub fn add_borrowed_books(&mut self, book: &mut Book) -> bool {
        if self.books_possessed.len() >= self.member_type.max_books as usize {
            println!("{} has already borrowed the max number of books", self.name);
            return false;
        }

        for possessed_book in &self.books_possessed {
            if book.isbn == possessed_book.clone() {
                println!("{} already has this book", self.name);
                return false;
            }
        }

        if self.debt > 0.0 {
            print!("{} has an outstanding debt of ${}. Please clear out debt first", self.name, self.debt);
            return false;
        }

        if matches!(book.status, BookStatus::BORROWED){
            println!("Book {} has already been borrowed", book.name);
            return false;
        }

        book.status = BookStatus::BORROWED;
        book.current_borrower_id = Some(self.member_id.clone());
        book.due_date = Some(Utc::now() + chrono::Duration::days(self.member_type.max_days.into()));

        self.books_possessed.push(book.isbn.clone());

        true

    }

    pub fn remove_borrowed_book(&mut self, book: &mut Book) -> bool {
        if self.books_possessed.is_empty(){
            println!("{} has not borrowed any books!", self.name);
            return false;
        }
        for (index, possesed_book) in self.books_possessed.iter().enumerate() {
            if *possesed_book == book.isbn {
                book.status = BookStatus::AVAILABLE;
                book.current_borrower_id = Option::None;
                self.books_possessed.remove(index);
                println!("{} has returned the book {}", self.name, book.name);

                if let Some(due) = book.due_date {
                    let elapsed = Utc::now() - due;

                    if elapsed.num_days() > 0 {
                        let fine = elapsed.num_days() as f32 * self.member_type.fine_rate;
                        self.debt += fine;
                        println!("{} returned the book late by {} days. Fine: ${}", self.name, elapsed.num_days(), fine);
                    }
                }
                                

                return true;
            }
        }

        false
    }

    pub fn pay_debt (&mut self, amount: f32) {
        if amount < 0.0{
            println!("The amount paid cannot be negative");
        }
        else if amount > self.debt{
            println!("The amount paid cannot be more than the debt owned");
        }
        else {
            self.debt -= amount;
        }
    }

    pub fn reserve_book(&mut self, book: &Book){
        if self.debt > 0.0 {
            println!("Pay off your debt first");
            return;
        }

        for possessed_book in &self.books_possessed{
            if *possessed_book == book.isbn {
                println!("You have already borrowed this book");
                return;
            }
        }

        for reserved_book in &self.reserved_books{
            if *reserved_book == book.isbn{
                println!("You have already reserved this book");
                return;
            }
        }

        self.reserved_books.push(book.isbn.clone());
    }

    pub fn cancel_reservation(&mut self, book: &Book){
        
        for (index, reserved_book) in self.reserved_books.iter().enumerate() {
            if *reserved_book == book.isbn {
                self.reserved_books.remove(index);
                return;
            }
        }

        println!("{} hasn't reserved this book", self.name);
    }

}