use crate::player::Player;
use colored::Colorize;
use std::io;

pub fn get_input() -> String {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read the line");
    inp.trim().to_string()
}

pub fn get_player_turn(username: &str) -> String {
    println!("Print your turn, {}", username);
    get_input()
}

pub fn notify_game_starts(players: &Vec<Player>) {
    print!("The russian roulette game starts now! Welcome, ");
    for player in players {
        print!("{}, ", player.name)
    }
    print!("\n")
}

pub fn notify_player_is_dead(name: &str) {
    println!("Player {} is {}", name, format!("dead").bold().red())
}

pub fn notify_blank_shoot() {
    println!("Nothing happened, congrats for this time")
}

pub fn notify_incorrect_input() {
    println!("Try to type again")
}
