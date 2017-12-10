use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut input = String::new();
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        let interpreter_name = "ro> ".as_bytes();
        stdout.write(interpreter_name).unwrap();
        stdout.flush().unwrap();

        stdin.read_line(&mut input).unwrap();
        println!("{:?}", input);
    }
}
