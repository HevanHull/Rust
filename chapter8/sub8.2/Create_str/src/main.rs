fn main() {
    let x = String::new();  // This creates an new empty string

    let data = "initial contents";

    let x = data.to_string();


    let y = "initial contents".to_string();
    // the code above and below are equivalent

    let z = String::from("initial contents");

    println!("{} === {} === {}", x, y, z);
}