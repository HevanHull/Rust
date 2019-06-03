fn main() {
    let ss = "Hello, World";

    let s1 = &ss[..5];
    let s2 = &ss[6..];
    println!("{}{}", s1, s2);
}
