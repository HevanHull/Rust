fn main() {
    let x = times_ten(10);
    println!("x's value is {}", x);
}

fn times_ten(x:u32) -> u32{
    x * 10
}