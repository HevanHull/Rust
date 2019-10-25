fn main() {
    enum Coin{
        Nickel,
        Dime,
        Quarter,
        Loonie,
        Toonie
    }

    fn value_in_coin (coin: Coin) -> f32 {
        match coin {
            Coin::Nickel => {
                println!("NickelBack is the Worst!");
                0.05
            },
            Coin::Dime => {
                println!("Chris Paul might have passed you this");
                0.10
            },
            Coin::Quarter => {
                println!("1/4 of an actual $1, poor peasant :(");
                0.25
            },
            Coin::Loonie => {
                println!("You must be a rich person");
                1
            },
            Coin::Toonie => {
                println!("You must be a wealth person");
                2
            }
        } 
    }
    println!("{}", value_in_coin(Coin::Dime));
    
}
