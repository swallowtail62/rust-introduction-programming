use std::fmt;
use std::fmt::Formatter;

enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

impl From<std::io::Error> for MyError {
    fn from(cause: std::io::Error) -> Self {
        MyError::Io(cause)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "I/O Error: {}", e),
            MyError::Num(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(MyError::from)?;
    num_str.trim().parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| MyError::Num(e))
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
