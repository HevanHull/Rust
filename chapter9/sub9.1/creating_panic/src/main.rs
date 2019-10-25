fn main() {
    panic!("Crash and Burn"); // here we intentionally cause a error if this code was to run
}
// There are Two errors in Rust: recoverable and unrecoverable 
// Recoverable errors has the type of "Result<T, E>"
// The "panic!" macro is used for unrecoverable errors
// a recoverable error maybe a file that is not found
// a unrecoverable is indexing beyond an array

// panic works in two ways unwinding where everything is cleaned up by rust
// and aborting which we leave to the operating system, if we have to have a small binary