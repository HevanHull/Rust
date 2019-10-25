use std::collections::HashMap;

fn main() {
    overwrite();
    entry__or_insert();
    old_value();
}

fn overwrite() {
    let mut pants = HashMap::new();

    pants.insert(String::from("sweats"), 20);
    pants.insert(String::from("sweats"), 3); // We can overwrite value

    println!("{:?}", pants) // {"sweats": 3}
}

fn entry__or_insert() {
    let mut shoes = HashMap::new();

    shoes.insert(String::from("Air Force 1s"), 1000);

    shoes.entry(String::from("Superstar")).or_insert(30); // we use the "entry" method as it takes a "key" as aparameter to check for an value
    // We use the "or.insert" method to give a value if the key doesnt have a value
    shoes.entry(String::from("Air Force 1s")).or_insert(20); // This doesnt change anything as "Air Force 1s" already has a value associated to a key
    // the return value of entry is "Entry" that is a enum
    // the "or_insert" method is to return

    println!("{:?}", shoes);
}

fn old_value() { // here we are counting how many words appear in the variable "text" using hashmaps
    let text = "hello world wonderful world"; // these words will be used a keys

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // "or.insert" reutrns a mutable reference
        *count += 1; // here we dereference to allow a value to be assigned to count using "*"
    }
    println!("{:?}", map); // {"hello": 1, "world": 2, "wonderful": 1}
}
