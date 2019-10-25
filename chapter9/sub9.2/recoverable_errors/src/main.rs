use std::fs::File;

fn main() {
    let f = File::open("hello.txt"); // Here we are attempting to open up a file "hello.txt"

    let f = match f { // we initaite a match case
        Ok(file) => file, // If the file is found return the file
        Err(error) => { 
            panic!("Problem opening that file {:?}", error) //else an error has been done
        },
    };
}
