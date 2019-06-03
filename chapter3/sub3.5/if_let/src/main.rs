fn main() {
    let condition: bool = false;
    let number = if condition {
        "1"
    }
    else { 
        "i"
    };
    println!("number is {}", number);
}
