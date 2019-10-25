fn main() {
    enum Spreadsheetcell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Spreadsheetcell::Int(3),
        Spreadsheetcell::Float(26.4),
        Spreadsheetcell::Text(String::from("Whole Lotta Red")),
    ];
}
// Here we create a vector that is using different types by holding an enum