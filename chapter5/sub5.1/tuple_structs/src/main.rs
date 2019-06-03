fn main() {
    struct XYZPlane(i32, i32, i32);
    struct RGB(i32, i32, i32);

    let onetwoone = XYZPlane(1, 0, 1);

    println!("x is equal to: {}", onetwoone.0);
}
