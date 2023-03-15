use crate::{
    player::Player,
    revolver::{Revolver, Shoot},
    client::Client
};

pub struct Game<'a, T: Client> {
    pub players: Vec<Player<T>>,
    pub revolver: &'a mut Revolver,
    alive_left: usize,
}

impl<'a, T: Client> Game<'a, T> {
    pub fn new(players: Vec<Player<T>>, revolver: &'a mut Revolver) -> Self {
        let alive_left = players.len();
        Game {
            players,
            revolver,
            alive_left,
        }
    }
    pub fn play(&mut self) {
        loop {
            for player in &mut self.players {
                if player.alive {
                    loop {
                        let result = player.make_turn(self.revolver);
                        match result {
                            Some(Shoot::DEADLY) => {
                                player.alive = false;
                                player.client.as_ref().notify_player_is_dead(&player.name);
                                self.alive_left -= 1;
                                if self.alive_left == 1 {
                                    return;
                                };
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
