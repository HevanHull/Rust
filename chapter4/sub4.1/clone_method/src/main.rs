fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
// Pointers, Length, Capacity
// both s1 & s2 source from the heap instead of making indiviual copies