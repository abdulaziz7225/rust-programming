#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

fn new_edition(book: &mut Book) {
    book.year = 2025;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immutable_book = Book {
        author: "J.R.R. Tolkien",
        title: "The Lord of the Rings",
        year: 1954,
    };

    let mut mutable_book = immutable_book;

    borrow_book(&immutable_book);

    borrow_book(&mutable_book);

    new_edition(&mut immutable_book);

    new_edition(&mut mutable_book);
}
