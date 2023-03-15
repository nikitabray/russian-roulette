use crate::client::Client;
use crate::revolver::{Revolver, Shoot};

pub struct Player<T: Client> {
    pub name: String,
    pub alive: bool,
    pub client: Box<T>,
}

impl<T: Client> Player<T> {
    pub fn make_turn(&self, revolver: &mut Revolver) -> Option<Shoot> {
        let input = self.client.get_player_turn(&self.name);
        match input.parse::<u8>().unwrap_or(0) {
            1 => return Some(revolver.shoot()),
            2 => {
                revolver.spin();
                return None;
            }
            _ => return None,
        };
    }
}
impl<T: Client> Player<T> {
    pub fn new(name: &str, client: T) -> Player<T> {
        Player {
            name: String::from(name),
            alive: true,
            client: Box::new(client),
        }
    }
}

impl<T: Client> PartialEq for Player<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
