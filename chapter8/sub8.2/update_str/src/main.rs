fn main() {
    let mut s = String::from("foo");
    s.push_str("bar"); // "push_str" method takes a string slice instead of ownership

    println!("{}", s); // foobar

    let mut s1 = String::from("foo");
    let s2 = "bar"; 
    s1.push_str(s2); // doesnt take ownership, s2 still reusable
    println!("{}", s1); // foobar

    character_push_to_string();
    concatenate();
}

fn character_push_to_string() {
    let mut b = String::from("lo");
    b.push('l'); // we can push a character to a string
    println!("{}", b);
}

fn concatenate() { 
    let n1 = String::from("tic")
    let n2 = String::from("tac")
    let n3 = String::from("toe")

    let m = s1 + "-" + &s2 + "-" + &s3;

    let m1 = format!("{}-{}-{}"s1, s2, s3); // "format!" works th same as "println!" but doesn't print the ouput
}

