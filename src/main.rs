extern crate scanner;

use scanner::lexer;

use std::io::Read;
use std::fs::File;

const FILE_NAME: &'static str = "test";

fn read_file() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    let mut file = File::open(&FILE_NAME)?;
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() {
    let buf = read_file().unwrap();
    let mut lexer = lexer::Lexer::new(&buf);

    for token in lexer {
        println!("{:?}", token);
    }
}
