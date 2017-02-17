extern crate interpreter;

use interpreter::lexer;
use interpreter::real;

use std::io::Read;
use std::fs::File;

const FILE_NAME: &'static str = "tests/random.txt";

fn read_file() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    let mut file = File::open(FILE_NAME)?;
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() {
    let r = real::Real::parse("30.25").unwrap();
    let f = real::Real::parse("5.5").unwrap();
    println!("{} + {} = {}", r, f, r / f);
    // let buf = read_file().unwrap();
    // let lexer = lexer::Lexer::new(&buf);

    // for (item, line, pos) in lexer {
    //     match item {
    //         Ok(token) => println!("ln: {} col: {}\n\t {:?}", line, pos, token),
    //         Err(e) => e.print_err(line, pos),
    //     }
    // }
}
