/*
Recoverable errors use the Result enum

enum Result<T, E> {
    Ok(T),
    Err(E)
}
*/
use std::fs;
use std::io;
fn main() {
    let contents = fs::read_to_string("the_ultimate_question.txt")
        .unwrap_or(String::from("There is no file in the path..."));
    println!("The contents of this file are: {:?}", contents);

    //using the unwrap_or() give you the option to have a place holder in the case that there is a None value, and None cannot be unwrapped.
    // You could use the expect(), but that will still cause the program to panic if there is no expected value, but you can give it a &str to give the user a response
    // You can also use a match expression, since Result is an enum:

    //Doing it this way is in turn the same way we use the unwrap_or()
    let result = fs::read_to_string("lkjlkjlkj.txt");
    let content = match result {
        Ok(message) => message,
        Err(error) => String::from("There is no file in the path with this name..."),
    };

    println!("contents is {}", content);

    // Let's try something else:

    let result = fs::read_to_string("lkjlkjlkj.txt");
    let content = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found."),
            io::ErrorKind::PermissionDenied => String::from("Permission Denied."),
            _ => panic!("Another type of error {:?}.", error),
        },
    };

    println!("contents is {}", content)
}
