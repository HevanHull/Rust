fn main() {
    struct User {
        age: u32,
        email: String,
        username: String,
        address: String,
        active: bool,
        points: u64
    }

    let user257 = User {
        age: 56,
        points: 10098,
        ..User
    };
}
