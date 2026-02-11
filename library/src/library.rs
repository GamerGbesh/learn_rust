use chrono::{DateTime, Utc};

use crate::book::Book;

pub struct Library {
    name: String,
    address: String,
    contact: String,
    email: String,
    established_date: DateTime<Utc>,
    categories: Vec<String>,
    books: Vec<Book>,
    members: Vec<String>
}


impl Library {
    pub fn new(name: String, address: String, contact: String, email: String) -> Self {
        Library {
            name,
            address,
            contact,
            email,
            established_date: Utc::now(),
            categories: Vec::new(),
            books: Vec::new(),
            members: Vec::new()
        }
    }

    pub fn initialize_categories(&mut self){
        self.categories.push(String::from("Fiction"));
        self.categories.push(String::from("Non-fiction"));
        self.categories.push(String::from("Science"));
        self.categories.push(String::from("History"));
        self.categories.push(String::from("Biography"));
        self.categories.push(String::from("Tech"));
        self.categories.push(String::from("Art"));
        self.categories.push(String::from("Music"));
        self.categories.push(String::from("Philosophy"));
        self.categories.push(String::from("Religion"))
    }

    pub fn add_category(&mut self, category: String){
        if self.categories.contains(&category){
            println!("This category already exists!");
            return;
        }
        self.categories.push(category);
    }

    pub fn add_book(&mut self, book: Book){
        if self.books.contains(&book){
            println!("This book already exists");
            return
        }
        self.books.push(book);
    }

    pub fn search_books(&self, isbn: String) -> Option<&Book>{
        for book in &self.books {
            if *book == isbn {
                return Some(book)
            }
        }
        return None
    }

    pub fn view_books(&self){
        println!("These are all the books in the library!");
        println!("Title    Author    Category");

        for (i, book) in self.books.iter().enumerate() {
            println!("{}. {}  {}  {}", i, book.name, book.author, book.category)    
        }

        println!("Done!");
    }

    pub fn add_member(&mut self, member: String){
        if self.members.contains(&member){
            println!("Already a member");
            return;
        }

        self.members.push(member);
    }

    pub fn display_members(&self){
        println!("Showing all members");
        println!("Current count: {}", self.members.len());

        for (i, member) in self.members.iter().enumerate() {
            println!("{}. {}", i, member);
        }
        println!("Done");
    }


}