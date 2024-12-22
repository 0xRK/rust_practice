use std::fs::File;
use std::io::{ErrorKind, self};
use std::fs;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // greeting_file = File::open("hello.txt").unwrap();

    // greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    let username = read_username_from_file().unwrap();

    println!("{username:?}");

    let last_char = last_char_of_first_line("hello\nworld\n").unwrap_or(' ');

    println!("{last_char:?}");
}


fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


