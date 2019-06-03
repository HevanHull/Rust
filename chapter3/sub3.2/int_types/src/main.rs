fn main() {
    let numb = "-124";
    let numb : i16 = numb.parse().expect("This doesnt work");
    let numb2 = "255";
    let numb2 : i16 = numb2.parse().expect("We got an error Boyz");
    let total: i16 = numb + numb2;
    println!("you scored a {}", total);
}
// result: you scored a 131