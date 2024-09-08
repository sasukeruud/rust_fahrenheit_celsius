use std::io;

fn main() {
    println!("Hello, world!");
    println!("Welcome to converter for temperature");

    loop {
        println!("q for quiting");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match input.as_str() {
            "q" => break,
            _ => continue
        }

    }
}
