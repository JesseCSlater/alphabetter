use rand::Rng;
use std::io;
use std::io::Write;

const MODES: [&str; 1] = ["1"];

fn main() {
    println!("Welcome to alphabetter!");
    println!("Use ^c to quit at any time.");
    println!("Choose your mode.");
    println!("(1) Number to Letter");
    let mut input = String::new();
    let input_instruction = "Input the number of a mode.";
    loop {
        request_input(">", &mut input);
        match parse_input(&input, &MODES) {
            Ok(_) => break,
            Err(e) => match e {
                ParseError::ToManyInputs => println!("To Many Inputs; {input_instruction}"),
                ParseError::NotInLanguage => println!("Invalid Input; {input_instruction}"),
            },
        }
    }
    println!("Type the letter corresponding to the given number.");
    let input_instruction = "Input a single lowercase letter.";
    loop {
        let number = rand::thread_rng().gen_range(1..=26);
        println!("---{number}---");
        loop {
            print!(">");
            io::stdout().flush().expect("Error printing");
            input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading input");
            match parse_input(&input, &ALPHABET) {
                Ok(n) => {
                    if n + 1 == number {
                        println!("Correct!");
                        break;
                    } else {
                        println!("Incorrect.");
                    }
                }
                Err(e) => match e {
                    ParseError::ToManyInputs => println!("To Many Inputs; {input_instruction}"),
                    ParseError::NotInLanguage => println!("Invalid Input; {input_instruction}"),
                },
            }
        }
    }
}

const ALPHABET: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];
const ARRAY26: [&str; 26] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17",
    "18", "19", "20", "21", "22", "23", "24", "25", "26",
];

#[derive(Debug)]
enum ParseError {
    ToManyInputs,
    NotInLanguage,
}

fn parse_input(input: &str, base: &[&str]) -> Result<usize, ParseError> {
    let mut input = input.clone();
    input = input.trim();
    if input.contains(char::is_whitespace) {
        return Err(ParseError::ToManyInputs);
    }
    base.iter()
        .position(|&s| s == input)
        .ok_or(ParseError::NotInLanguage)
}

fn request_input(prompt: &str, input: &mut String) {
    print!("{prompt}");
    io::stdout().flush().expect("Error printing");
    *input = String::new();
    io::stdin().read_line(input).expect("Error reading input");
}
