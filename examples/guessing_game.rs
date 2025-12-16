use figlet_rs::FIGfont;
use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, Write},
};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let small_font = FIGfont::from_file("assets/fonts/Cosmike.flf").expect("Missing font");
    let title = small_font
        .convert("Guess the number")
        .expect("Font convert failed");

    // start game
    println!("\n{}", title);

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please try again.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                let game_win_message = small_font.convert("You win").expect("Font convert failed");

                println!("\n{}", game_win_message);
                break;
            }
        }
    }
}
