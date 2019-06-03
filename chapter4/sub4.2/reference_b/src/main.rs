fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The string : {}; has a length of {}", s1, len);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}
// "&" used for referencing
// "*" used for dereference 