use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Orange"), 80);
    scores.insert(String::from("Brown"), 90);

    let team_name = String::from("Orange");
    let score = scores.get(&team_name); // how to access the "value" pair for Orange (Key).

    println!("{:?}", scores) // how to output the hashmap from scores 
    println!("{:?}", score); // how to ouput the key's (Orange) value (80)

    iterate();
}

fn iterate() {
    let mut socks = HashMap::new();

    socks.insert(String::from("Stripped"), 10);
    socks.insert(String::from("Plain"), 3);

    for (key,value) in &socks {
        println!("{}: {}", key, value); 
    } 
    // We can iterate a HashMap to output the Key and Value pairs
}
