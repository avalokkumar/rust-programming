mod player;
mod menu;
mod song;

use crate::menu::Menu;
use crate::player::Player;
use crate::song::Song;

fn main() {
    let mut player = Player::new();
    let mut menu = Menu::new();

    loop {
        menu.print_menu();
        let choice = menu.get_choice();

        match choice {
            1 => {
                let title = input_string("Enter song title: ");
                let artist = input_string("Enter artist name: ");
                let duration = input_u32("Enter song duration (in seconds): ");
                let song = Song::new(title, artist, duration);
                player.add_song(song);
                println!("Song added successfully!");
            }
            2 => player.play(),
            3 => player.pause(),
            4 => player.stop(),
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn input_string(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn input_u32(prompt: &str) -> u32 {
    loop {
        let input = input_string(prompt);
        match input.parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}