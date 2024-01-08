use std::str::FromStr;

fn main() {
    let drop_string = drop_box::DropBox::new("UwU".to_owned(), |s| {
        println!("Drop function called for {}", s)
    });
    println!("My drop string is: {}", drop_string.to_owned());
}
