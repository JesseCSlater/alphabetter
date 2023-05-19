use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to alphabetter!");
    println!("Use ^c to quit.");
    println!("Type the letter corresponding to the given number.");

    loop {
        let number = rand::thread_rng().gen_range(1..=26);
        println!("---{number}---");
        let mut input;
        let mut letter: u8;
        loop {
            print!(">");
            io::stdout().flush();
            input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading input");
            letter = match input.bytes().next() {
                Some(byte) => byte,
                None => {
                    println!("Enter a single lowercase letter.");
                    continue;
                }
            };
            if letter < 97 || letter > 122 {
                println!("Enter a single lowercase letter.");
                continue;
            };
            if letter - 96 == number {
                println!("Correct");
                break;
            };
        }
    }
}
