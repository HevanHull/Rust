fn main() {
    let v = vec![100, 87, 99];
    for i in &v { 
        println!("{}", i);
    }

    let mut w = vec![80.0, 90.1, 94.0];
    for q in &mut w {
        *q *= 1.05;
        println!("{}", q);
    }
}
 // we use the deference operator "*" to get the value of "q"