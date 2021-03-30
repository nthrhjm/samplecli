use std::fmt;

enum MyErr {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyErr::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyErr::Num(cause) => write!(f, "Parse Error: {}", cause),
        }
    }
}

fn get_int_from_file() -> Result<i32, MyErr> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path).map_err(|e| MyErr::Io(e))?;
    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| MyErr::Num(e))
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        // Err(e) => match e {
        // MyErr::Io(cause) => println!("I/O Err: {}", cause),
        // MyErr::Num(cause) => println!("Parse Err: {}", cause),
        // },
        Err(e) => println!("{}", e),
    }
}
