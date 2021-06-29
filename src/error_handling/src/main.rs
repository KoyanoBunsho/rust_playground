use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    let g = File::open("hello.txt").expect("Failed to open hello.txt");
}
