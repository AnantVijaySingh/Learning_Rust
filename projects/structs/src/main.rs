
struct Book {
    number: u32,
    name: String,
    authors: String,
    goodreads_link: String
}

//Tuple struct
struct Color (i32, i32, i32);

fn main() {
    let book_1 = Book {
        number: 1,
        name: String::from("Beyond Entrepreneurship 2.0"),
        authors: String::from("James C. Collins, Bill Lazier"),
        goodreads_link: String::from("www.goodreads.com/book/show/55270310-be-2-0-beyond-entrepreneurship-2-0")
    };

    display_book_details(&book_1);
    println!("\n");

    let book_2 = build_book_struct(
        2,
        String::from("Good Economics for Hard Times"),
        String::from("Abhijit V. Banerjee, Esther Duflo"),
        String::from("www.goodreads.com/book/show/51014619-good-economics-for-hard-times")
    );

    display_book_details(&book_2);

    let black = Color(0, 0 ,0);
}

fn build_book_struct (number: u32, name: String, authors: String, goodreads_link: String) -> Book {
    Book {
        // Using the Field Init Shorthand
        number,
        name,

        // Not using the Filed Init Shorthand
        authors: authors,
        goodreads_link: goodreads_link
    }
}

fn display_book_details(book: &Book) {
    println!("Name:            {}", book.name);
    println!("Author(s):       {}", book.authors);
    println!("GoodReads link:  {}", book.goodreads_link);
}