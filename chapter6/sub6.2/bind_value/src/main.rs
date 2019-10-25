#[derive(Debug)]
fn main() {
    enum Province{
        Ontario,
        Quebec,
        Alberta,
        British Columbia
    }

    enum Coin{
        Nickel,
        Dime,
        Quarter,
        Loonie,
        Toonie
    }

    fn Value_in_cents (coin: Coin) -> f32 {
        match coin{ 
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Provincial quarter from {:?}!", Province);
                25
            },
            Coin::Loonie => 100,
            Coin::Toonie => 200,
        }
    }
}
