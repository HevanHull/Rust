fn main() {
    struct User{
        age: u32,
        email: String,
        username: String,
        address: String,
        active: bool,
        points: u64
    }

    let mut user25 = User{
        age: 45,
        email: String::from("HevanHull@gmail.com"),
        username: String::from("Hevan_Hull"),
        address: String::from("None of your Business Street"),
        active: true,
        points: 999999999999999,
    };
    println!("Email: {}", user25.email);

    user25.email = String::from("HevanHull1@gmail.com");

    println!("Email: {}", user25.email);
}

// "{}"" <-- fields