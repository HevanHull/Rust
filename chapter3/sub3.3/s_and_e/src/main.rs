fn main() {
    let x = 5;

    let z = {
        let x = 10;
        x + 3
    };
    println!("Value of z is {}", z);
}
