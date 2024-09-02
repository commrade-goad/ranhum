use std::io::{self, Write};

pub fn read_stdin(str: &str) -> String{
    let mut line: String = String::new();
    print!("{}", str);
    _ = io::stdout().flush();
    io::stdin().read_line(&mut line).unwrap();
    return line;
}
