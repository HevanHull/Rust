struct Three_D_Object {
    x: u32,
    y: u32,
    z: u32
}

fn main() {
    
    let prism1 = Three_D_Object {
        x: 30, 
        y: 40, 
        z: 50
    };
    
    println!(
        "Total volume of parameters given is: {}cubed", 
        volume(&prism1)
    );

}

fn volume(three_D_Object: &Three_D_Object) -> u32 {
    three_D_Object.x * three_D_Object.y * three_D_Object.z
}