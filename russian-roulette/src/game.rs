use crate::{
    player::Player,
    revolver::{Revolver, Shoot},
    ui::{get_player_turn, notify_blank_shoot, notify_incorrect_input, notify_player_is_dead},
};

pub struct Game<'a> {
    pub players: Vec<Player>,
    pub revolver: &'a mut Revolver,
}

impl<'a> Game<'a> {
    pub fn play(mut self) {
        loop {
            let alive: Vec<&Player> = self.players.iter().filter(|player| player.alive).collect();
            if alive.len() == 0 {
                println!("Bye, losers!");
                break;
            };
            for player in &mut self.players {
                if player.alive {
                    loop {
                        let input = get_player_turn(&player.name);
                        let result = player.make_turn(self.revolver, &input);
                        match result {
                            Some(Shoot::DEADLY) => {
                                player.alive = false;
                                notify_player_is_dead(&player.name);
                                break;
                            }
                            Some(Shoot::BLANK) => {
                                notify_blank_shoot();
                                break;
                            }
                            _ => notify_incorrect_input(),
                        }
                    }
                }
            }
        }
    }
}
