fn main() {
    let a = vec![4, 4, 5, 7];

    a[999]; // This code is to fail because we are indexing to the 1000th element in the array
    // This causes a panic 
}
// To see where a panic orginated at we can run "RUST_BACKTRACE=1 cargo run" 
// to see each function that was called that lead to this point 