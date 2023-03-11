use crate::client::{Client, LocalClient};
use crate::revolver::{Revolver, Shoot};

pub struct Player {
    pub name: String,
    pub alive: bool,
    pub client: Box<dyn Client>,
}

impl Player {
    pub fn make_turn(&self, revolver: &mut Revolver) -> Option<Shoot> {
        let input = self.client.get_player_turn(&self.name);
        match input.parse::<u8>().unwrap_or(0) {
            1 => return Some(revolver.shoot()),
            2 => {
                revolver.spin();
                return None;
            },
            _ => return None,
        };
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: Default::default(),
            alive: true,
            client: Box::new(LocalClient {}),
        }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
