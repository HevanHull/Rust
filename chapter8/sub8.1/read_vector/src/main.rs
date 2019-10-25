fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let fourth: &i32 = &v[3]; //<-- We use the "reference" and indexing to get the fourth element
    println!("The fourth element is {}", fourth);
    
    match v.get(3) { //<-- We use the "get" method to get the fouth element
        Some(fourth) => println!("The fourth element is {}", fourth),
        None => println!("Fourth element cease to exist"),
    }
}
//Note:
// Accessing an no-existent element will cause a crash
// to control for that case use Option<&T> with a None value