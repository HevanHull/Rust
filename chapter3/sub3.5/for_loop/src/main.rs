fn main() {
    let a = [50, 40, 30, 20, 10];

    for element in a.iter(){
        println!("the value is {}", element);
    }
}
//The "while" alternative is:
//fn main() {
//  let a = [10, 20, 30, 40, 50];
//  let mut index = 0;
//
//  while index < 5 {
//  println!("the value is: {}", a[index]);
//
//      index += 1;
// }
//}
