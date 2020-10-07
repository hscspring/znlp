use std::collections::HashMap;
fn insert() {
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
}

fn main() {
    #[derive(Debug)]
    struct Entry {
        weight: f32
    };

    #[derive(Default, Debug)]
    struct SomeOptions {
        foo: i32,
        bar: f32,
    }

    let _x = Entry { weight: 1.2 };
    let options: SomeOptions = Default::default();

    println!("{:?}", options);

    let x = insert();
    println!("{:?}", x);
}