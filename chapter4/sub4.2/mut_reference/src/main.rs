fn main() {
    let mut w = String::from("Where's");

    change(&mut w);
    println!("{}", w);
}

fn change(some_string: &mut String) {
    some_string.push_str(", Waldo");
}

// cant borrow an immutable and mutable, only after one var becomes used
//cant borrow two mutable as one has too be out of scope
// can borrow two immutable as it is just reading the data
//At any given time, you can have either one mutable reference or any number of immutable references