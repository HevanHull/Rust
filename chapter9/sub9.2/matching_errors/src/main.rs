use std::fs::File;
use std::io::ErrorKind;

fn main() { // in here we plan for the case in which "hello.txt" cant be found
    let f = File::open("hello.txt");
    primitive();
    // concise();
}

fn primitive() {
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() { // io::Error returns a Struct. we can call the the method "kind" to get the enum "io::ErrorKind"
            ErrorKind::NotFound => match File::create("hello.txt") { // We create a "hello.txt" if one isnt found
                Ok(fc) => fc, 
                Err(e) => panic!("Problem opening the file {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", error),
        },
    };
}

fn concise() { // more concise way of doing what is stated up above
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() ==  ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
              })
        }   else {
                panic!("Problem opening the file {:?}", error);
        }
    });
}