use rand::Rng;
use std::collections::HashMap; //std library is already built in no need to call a dependency

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("{}", secret_number);
}
