fn main() {
    for c in "नमस्ते".chars() { // we can iterate characters of the string
        println!("{}", c); // should show 4 elements, technically 6 but the 4 and 6 elements are not letters   
    }

    for b in "नमस्ते".bytes() { // we can iterate bytes of the string
        println!("{}", b); //should return 18 bytes that make up "namaste"
    }
}
