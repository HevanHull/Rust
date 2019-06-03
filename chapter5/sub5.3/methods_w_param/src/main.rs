struct Prism {
    length: u32,
    width: u32,
    height: u32
}
impl Prism{
    fn volume (&self) -> u32{
        self.length * self.width * self.height
    }

    fn can_hold (&self, other: &Prism) -> bool {
        self.length > other.length &&
        self.width > other.width &&
        self.height > other.height
    }
}
fn main() {
    let prism1 = Prism {
        length: 10, 
        width: 10, 
        height: 10
    };

    let prism2 = Prism {
        length: 30, 
        width: 30, 
        height: 30
    };

    println!("It is {} that prism2 can hold prism1", 
        prism2.can_hold(&prism1)
    );

}
