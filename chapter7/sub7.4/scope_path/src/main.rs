fn main() {

    mod front_of_house{
        pub mod hosting{
            pub fn check_in(){
                println!("You have to wait!");
            }
        }
    }

    use crate::front_of_house::hosting;
    //Relative: "use self::front_of_house::hosting;"

    
    pub fn calling(){
        hosting::check_in();
    }
}
