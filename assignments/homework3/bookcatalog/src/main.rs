use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    
    // Create the new file
    let mut file = File::create(filename);

    let writer = Write::write(&mut file, ""); // Clear the file before writing
    // Save each book to the file per line and in the struct format
    for book in books {
        writer!(file, "{}|{}|{}", book.title, book.author, book.year);
    }
}

 
fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    // Open the file and read it line by line
    // We also need to parse it into the struct format and return a vector of books
    let file = File::open(filename).expect("Where's my file? Can't find books.txt");
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    // Read each line and parse it the Book struct
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 3 {
            let book = Book {
                title: parts[0].to_string(),
                author: parts[1].to_string(),
                year: parts[2].parse().unwrap_or(0),
            };
            books.push(book);
        }
    }
    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}