fn main() {
    let int: i32 = 8;
    
    if int % 6 == 0{
        println!("{:x} (hexadecimal) is divisible by 6", int);
    } 
        else if int % 4 == 0{
            println!("{:b} (binary) is divisible by 4", int);
        }
        else{
            println!("no number can divide {}", int);
        }
    println!("Hello, world!");
}
