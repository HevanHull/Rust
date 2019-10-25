fn main() {
    fn serve_order() {}

    mod back_of_house {
        fn fix_incoreect_order() {
            super::serve_order();
            cook_order();
        }
        fn cook_order() {}
    }
}
// Super allows us to go to the grandparent module
// in this case it's going to be "crate"