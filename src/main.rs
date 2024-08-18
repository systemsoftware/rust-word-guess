use colored::*;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut lang = String::new();

    let mut length = String::new();

    let mut guesses = 0;

    println!("Enter the length of the word you want to guess: ");
    std::io::stdin().read_line(&mut length).expect("Failed to read line");
    let mut length: u32 = length.trim().parse().expect("Please enter a number!");
    if length <= 1 {
        println!("The length must be greater than 1. Setting the length to a random number between 1 and 10.");
       length = rand::random::<u32>() % 10 + 1;
    }

    println!("Enter the language of the word you want to guess (en, es, fr, de, it, pt): ");
    std::io::stdin().read_line(&mut lang).expect("Failed to read line");
    let lang = lang.trim();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let response = reqwest::blocking::get(&format!("https://random-word-api.herokuapp.com/word?number=1&lang={}&length={}", lang, length))?
    .text()?;

let json: Value = serde_json::from_str(&response)?;

let random_word = json[0].as_str().unwrap();
        
    print!("The word is {} letters long.", random_word.len());
    println!("\nType 'help' for help, 'give_up' to give up, or 'len' to get the length of the word.");

    loop {
        let mut guess = String::new();
        println!(" ");
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim();

        if guess != "give_up" && guess != "len" && guess != "help" && guess != "cheat" {
            guesses += 1;
        for (i, c) in guess.chars().enumerate() {
            match random_word.chars().nth(i) {
                Some(r) if c == r => print!("{}", c.to_string().green()),
                _ => print!("{}", c.to_string().red()),
            }
        }        
    }

        match guess {
            "give_up" => {
                println!("\nThe word was: {}", random_word);
                break Ok(());
            }
            "len" => println!("\nThe word is {} letters long.", random_word.len()),
            _ if guess == random_word => {
                println!("\nYou guessed the word in {} guesses!", guesses);
                break Ok(());
            }
            "help" => println!("\nType 'give_up' to give up and 'len' to get the length of the word."),
            "cheat" => println!("\n{}", random_word),
            _ => continue,
        }
   
    }
}