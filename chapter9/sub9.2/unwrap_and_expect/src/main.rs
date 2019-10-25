use std::fs::File;

fn main() {
    let f = File::open("hello.text").unwrap(); // if the result value from "File::open" is ok then "unwrap" will return the value inside the ok
// if it is the "Err" variant then panic! would be called for us
    expect();
}

fn expect() {
    let h = File::open("hello.txt").expect("Failed to open the file: hello.txt") // the ".expect" allows us to choose the error message ourselves
}