use std::io;
use std::io::Write;
use rand::seq::SliceRandom;

fn main() {
    let words = vec!["apple", "banana", "orange", "strawberry", "watermelon"];
    let mut rng = rand::thread_rng();
    let chosen_word = words.choose(&mut rng).unwrap();

    let mut guessed_letters = vec!['-'; chosen_word.len()];
    let mut attempts = 6;

    println!("Welcome to Hangman!");

    loop {
        println!("Attempts left: {}", attempts);
        println!("Guessed word: {}", guessed_letters.iter().collect::<String>());

        print!("Enter a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim();

        if guess.len() != 1 {
            println!("Please enter a single letter.");
            continue;
        }

        let guess_char = guess.chars().next().unwrap();

        if guessed_letters.contains(&guess_char) {
            println!("You already guessed that letter.");
            continue;
        }

        let mut found = false;
        for (i, &c) in chosen_word.chars().enumerate() {
            if c == guess_char {
                guessed_letters[i] = c;
                found = true;
            }
        }

        if !found {
            attempts -= 1;
            println!("Incorrect guess!");
            if attempts == 0 {
                println!("You lost! The word was: {}", chosen_word);
                break;
            }
        }

        if guessed_letters.iter().all(|&c| c != '-') {
            println!("Congratulations! You guessed the word: {}", chosen_word);
            break;
        }
    }
}
