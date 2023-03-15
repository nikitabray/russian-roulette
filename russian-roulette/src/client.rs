use std::io;
use crate::player::Player;
mod messages {
    use colored::Colorize;

    pub fn generate_get_player_turn_message(username: &str) -> String {
        String::from(format!("Print your turn, {}\n1. Shoot\n2. Spin the wheel", username))
    }

    pub fn generate_game_starts_message(players: Vec<&str>) -> String {
        let mut message = String::from("The russian roulette game starts now! Welcome, ");
        for player in players.iter() {
            message.push_str(&format!("{} ", player));
        }
        message.push_str("\n");
        message
    }

    pub fn generate_player_is_dead_message(username: &str) -> String {
        String::from(format!(
            "Player {} is {}",
            username,
            "dead".bold().red()
        ))
    }

    pub fn generate_blank_shoot_message() -> String {
        String::from("Nothing happened, congrats for this time")
    }
}

pub struct AnonymousClient {}

pub trait Client {
    fn send_message(&self, message: &str);
    fn get_player_turn(&self, user_id: &str) -> String;
    fn notify_game_starts(&self, players: Vec<&str>);
    fn notify_player_is_dead(&self, user_id: &str);
    fn notify_blank_shoot(&self);
}

pub struct LocalClient {
}

impl LocalClient {
    pub fn new() -> Self {
        LocalClient {  }
    }
    fn get_input() -> String {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read the line");
        inp.trim().to_string()
    }
}

impl Client for LocalClient {
    fn send_message(&self, message: &str) {
        println!("{}", message);
    }
    fn get_player_turn(&self, name: &str) -> String {
        let message = messages::generate_get_player_turn_message(name);
        self.send_message(message.as_str());
        LocalClient::get_input()
    }

    fn notify_game_starts(&self, players: Vec<&str>) {
        let message = messages::generate_game_starts_message(players);
        self.send_message(message.as_str());
    }

    fn notify_player_is_dead(&self, name: &str) {
        let message = messages::generate_player_is_dead_message(name);
        self.send_message(message.as_str());
    }

    fn notify_blank_shoot(&self) {
        let message = messages::generate_blank_shoot_message();
        self.send_message(message.as_str());
    }
}
