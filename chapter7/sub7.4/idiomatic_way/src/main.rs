use std::fmt;
use std::io;

fn main() {


    fn function1() -> fmt::Result {
        Ok(())
    }

    fn function2() -> io::Result<()> {
        Ok(())
    }
}
//doing this allows us to use two different results, if we did it the non ruct way we would get an error