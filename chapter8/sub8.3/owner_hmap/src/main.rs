use std::collections::HashMap;

fn main() {
    let field_name = String::from("Artist");
    let field_value = String::from("Drake aka ChampagnePapi El Papi Drako");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // types like i32 are copied and not taken
    // String are moved and taken by the HashMap when using the insert method
    // We can can use refernces if we want to use reuse the variables
}