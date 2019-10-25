use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new(); // Here we create a HashMap "scores"

    scores.insert(String::from("blue"), 69); // Here we insert a String and i32 in the HashMap "Scores"
    scores.insert(String::from("red"), 70);
}

fn otherway() {
    let teams = vec![String::from("yellow"), String::from("pink")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // underscores are used to specify the key and values pair   
    // We can "zip" to create a "vector of tuples"
    // "Collect" can turn a vector of tuples into a hashmap

}
