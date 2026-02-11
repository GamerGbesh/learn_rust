use chrono::{DateTime, Utc};

pub enum BookStatus{
    AVAILABLE,
    BORROWED
}

pub struct Book {
    pub name: String,
    pub isbn: String,
    pub author: String,
    pub genre: String,
    pub category: String,
    pub status: BookStatus,
    pub due_date: Option<DateTime<Utc>>,
    pub edition: i8,
    pub publisher: String,
    pub publication_date: DateTime<Utc>,
    pub description: String,
    pub page_count: i32,
    pub language: String,
    pub current_borrower_id: Option<String>
}



impl Book {
    pub fn new(
        name: String,
        isbn: String,
        author: String,
        genre: String,
        category: String,
        edition: i8,
        publisher: String,
        publication_date: DateTime<Utc>,
        description: String,
        page_count: i32,
        language: String
    ) -> Self {
        Book {
            name,
            isbn,
            author,
            genre,
            category,
            status: BookStatus::AVAILABLE,
            due_date: None,
            edition,
            publisher,
            publication_date,
            description,
            page_count,
            language,
            current_borrower_id: None
        }
    }
}

impl PartialEq<String> for Book {
    fn eq(&self, other: &String) -> bool {
        self.isbn == *other
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}


impl Eq for Book{}