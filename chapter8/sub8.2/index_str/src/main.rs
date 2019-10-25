fn main() {
    let s1 = String::from("hello");
//  let e = s1[1]; // Rust doesnt allow for the indexing of Strings

    let len = String::from("Hola").len();
    println!("{}", len); // 4

    let len2 = String::from("Здравствуйте").len();
    println!("{}", len2); // 24 why? because each letter takes up 2 bytes specified in UTf-8 



    let hello = "Здравствуйте";

    let s = &hello[0..4]; // we can string slice however string slicing using "[0..1]" will cause a panic
    
    println!("{}", s); // Зд
}
