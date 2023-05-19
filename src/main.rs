use rand::Rng;
use std::io;
use std::io::Write;

const MODES: [&str; 2] = ["1", "2"];

fn main() {
    println!("Welcome to alphabetter!");
    println!("Use ^c to quit at any time.");
    let mut input = String::new();
    let input_instruction = "Input the number of a mode.";
    loop {
        println!("Choose your mode.");
        println!("(1) Number to Letter");
        println!("(2) Letter to Number");
        request_input(">", &mut input);
        match parse_input(&input, &MODES) {
            Ok(0) => matching_game(
                26,
                &ARRAY26,
                &ALPHABET,
                "number from 1 to 26",
                "lowercase letter",
            ),
            Ok(_) => matching_game(
                26,
                &ALPHABET,
                &ARRAY26,
                "lowercase letter",
                "number from 1 to 26",
            ),
            Err(e) => match e {
                ParseError::ToManyInputs => println!("To Many Inputs; {input_instruction}."),
                ParseError::NotInLanguage => println!("Invalid Input; {input_instruction}."),
            },
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

fn parse_input(input: &str, options: &[&str]) -> Result<usize, ParseError> {
    let mut input = input.clone();
    input = input.trim();
    if input.contains(char::is_whitespace) {
        return Err(ParseError::ToManyInputs);
    }
    options
        .iter()
        .position(|&s| s == input)
        .ok_or(ParseError::NotInLanguage)
}

fn request_input(prompt: &str, input: &mut String) {
    print!("{prompt}");
    io::stdout().flush().expect("Error printing");
    *input = String::new();
    io::stdin().read_line(input).expect("Error reading input");
}

fn matching_game(
    alpha_len: usize,
    clue_alpha: &[&str],
    guess_alpha: &[&str],
    clue_name: &str,
    guess_name: &str,
) {
    println!("Input the {guess_name} which cooresponds to the {clue_name}. How many can you get?");
    let mut score: usize = 0;
    let mut input = String::new();
    'outer: loop {
        let index = rand::thread_rng().gen_range(0..alpha_len);
        println!("---{}---", clue_alpha[index]);
        loop {
            request_input(">", &mut input);
            match parse_input(&input, guess_alpha) {
                Ok(n) => {
                    if n == index {
                        println!("Correct!");
                        score += 1;
                        println!("Your score is {score}.");
                        break;
                    } else {
                        println!("Incorrect :(");
                        println!("Your score was {score}.");
                        break 'outer;
                    }
                }
                Err(e) => match e {
                    ParseError::ToManyInputs => println!("To Many Inputs; Input a {guess_name}"),
                    ParseError::NotInLanguage => println!("Invalid Input; Input a {guess_name}"),
                },
            }
        }
    }
}
