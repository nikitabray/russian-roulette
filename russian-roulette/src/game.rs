use crate::{
    player::Player,
    revolver::{Revolver, Shoot},
};

pub struct Game<'a> {
    pub players: Vec<Player>,
    pub revolver: &'a mut Revolver,
}

impl<'a> Game<'a> {
    fn check_winner(&self) -> Option<&Player> {
        let alive: Vec<&Player> = self.players.iter().filter(|player| player.alive).collect();
        if alive.len() == 1 {
            return self.players.first();
        } else {
            None
        }
    }
    pub fn play(&mut self) {
        loop {
            let the_winner = self.check_winner();
            match the_winner {
                None => (),
                Some(player) => {
                    player
                        .client
                        .send_message(&format!("You won, {}", player.name));
                    return;
                }
            };
            for player in &mut self.players {
                if player.alive {
                    loop {
                        let result = player.make_turn(self.revolver);
                        match result {
                            Some(Shoot::DEADLY) => {
                                player.alive = false;
                                player.client.as_ref().notify_player_is_dead(&player.name);
                                break;
                            }
                            Some(Shoot::BLANK) => {
                                player.client.as_ref().notify_blank_shoot();
                                break;
                            }
                            _ => player.client.send_message("Incorrect input, try again"),
                        }
                    }
                }
            }
        }
    }
}
