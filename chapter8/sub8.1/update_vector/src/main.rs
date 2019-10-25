fn main() {
    let mut _v: Vec<i32> = Vec::new();

    let mut _v = vec![1, 2, 3];

    _v.pop();

    _v.push(-7);
    // <-- Vector in scope
    println!("{:?}", _v)
} // <-- Vector out of scope and freed up

//Output [1, 2, -7]