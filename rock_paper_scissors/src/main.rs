// 1. Create RPS struct
// 2. Implement methods to generate and compare 
// 3. Get input from user
// 4. Play
use rand::Rng;
use std::io::{self, Write};

#[derive(Debug)]
pub struct RPS {
    chosen: String
}

impl RPS {
    pub const CHOISES: [&'static str; 3] = ["rock", "paper", "scissor"];
    pub fn choose () -> RPS {
        let rand_index = rand::thread_rng().gen_range(0..3);
        let rand_choice = RPS::CHOISES[rand_index];
        RPS{chosen: String::from(rand_choice)}
    }
    pub fn get_input() -> Option<RPS> {
        print!("Rock, Ppaer, or Scissors? [\"q\" to quit]: ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        user_input = user_input.trim().to_lowercase();
        if user_input == "q" {
            println!("Quitting the game...");
            None
        }
        else {
            Some(RPS{chosen: user_input})
        }
        
    }
}

fn main() {
    let computer_move = RPS::choose();
    let user_input = RPS::get_input();
    user_input.unwrap_or(std::process::exit(0));
    println!("{:?}", computer_move);
    println!("{:?}", user_input);
}
