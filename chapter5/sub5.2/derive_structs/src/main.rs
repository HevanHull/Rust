#[derive(Debug)]
struct User {
        name: String,
        user_name: String,
        points: u64,
    }
    
    fn main() {
    
    let hevan = User {
        name: String::from("Hevan"),
        user_name: String::from("ImTheCashMan"),
        points: 100000000340,
    };

    println!("{:?}", hevan);
}
