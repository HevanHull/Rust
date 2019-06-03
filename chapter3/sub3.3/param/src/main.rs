fn main() {
    another_function(-9, 100);
}

fn another_function(x: i32, y:i32){
    let z: i32 = y + x;
    
    println!("x's value is {}", x);
    println!("y's value is {}", y);
    println!("z's value is {}", z);
}