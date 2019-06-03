fn main() {
    let mut number = 5;

    while number != 0{
        println!("T-MINUS {} !!!", number);
        
        number -= 1;
    }
    println!("ZERO!!");
    println!("LIFT OFF!!!");
}

// The "FOR" Alt is:
//fn main() {
    //for number in (1..4).rev() {
        //println!("{}!", number);
    //}
    //println!("LIFTOFF!!!");
//}