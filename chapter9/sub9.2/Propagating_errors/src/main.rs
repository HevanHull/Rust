use std::io;
use std::io::Read;
use std::fs::File;
use std::error::Error;

fn main() {
    read_username_from_file();
    lessComplex();

}

fn read_username_from_file() -> Result<String, io::Error> { // we set the return case to be a String or an Error
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file, // returns the file if the operation of opening "hello.txt" is successful
            Err(e) => return Err(e), // We return the error
        };

        let mut s = String::new(); 

        match f.read_to_string(&mut s) { // here the contents of f is read into s
            Ok(_) => Ok(s), // if the operation succeeds then we return the username that's in variable "s" 
            Err(e) => Err(e), // if not we return the error value
        }
}

fn short() -> Result<String, io::Error> { // has the same functionality as up above
    let mut f = File::open("hello.txt")?; // "?" operator works the same way as match. but "?" is the "from" function which is used to convert error types
    let mut s = String::new(); // The "?" can only be use in functions that return a "Result" or "Option"
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn shorter() -> Result<String, io::Error> { // here we shorten the code above by chaining methods
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn shortest() -> Result<String, io::Error> { // shorter but same as the code above in functionality
    fs::read_to_string("hello.txt") // "fs::read_to_string" function opens the file, 
    //creates a new String, reads the contents of the file, puts the contents into that String, and returns it

}

//fn main() -> Result<(), Box<dyn Error>> { // Box<dyn Error> type is trait object, as it can read "any type of error"
//    let f = File::open("hello.txt")?; // "?" is allowed use in main because of "Box<dyn Error>"
//
//    Ok(()) // Besides "Result", "()" is also a valid return type
//}