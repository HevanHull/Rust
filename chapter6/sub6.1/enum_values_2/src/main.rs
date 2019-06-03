fn main() {
    
    enum CarEngine{
        V4,
        V6,
        V8,
        V10,
        V12,
    }

    struct CarKind{
        engine: CarEngine,
        horsepower: u32,
        brand: String
    }

    let X_fi = CarKind{
        engine: CarEngine::V6,
        horsepower: 400,
        brand: String::from("BMW")
    };

    let e_fi_fi = CarKind{
        engine: CarEngine::V8,
        horsepower: 700,
        brand: String::from("Mercedes")
    };
}