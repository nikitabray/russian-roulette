use crate::{
    player::Player,
    revolver::{Revolver, Shoot},
};

pub struct Game<'a> {
    pub players: Vec<Player>,
    pub revolver: &'a mut Revolver,
}

impl<'a> Game<'a> {
    pub fn play(&mut self) {
        loop {
            let alive: Vec<&Player> = self.players.iter().filter(|player| player.alive).collect();
            if alive.len() == 0 {
                println!("Bye, losers!");
                break;
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
                            _ => println!("Incorrect input"),
                        }
                    }
                }
            }
        }
    }
}
