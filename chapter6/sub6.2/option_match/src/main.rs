fn main() {
    println!("Hello, world!");


    fn times_three (x: Option<i32>) -> Option<i32> {
        match x{
            None => None,
            Some(i)=> Some(i * 3)
        }
    }
}