use crate::revolver::{Revolver, Shoot};

pub struct Player {
    pub name: String,
    pub alive: bool,
}

impl Player {
    pub fn make_turn(&self, revolver: &mut Revolver, turn_type: &str) -> Option<Shoot> {
        match turn_type {
            "sh" | "shoot" | "shot" => return Some(revolver.shoot()),
            "sp" | "spin" | "reload" => {
                revolver.spin();
                return None;
            }
            &_ => None,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: Default::default(),
            alive: true,
        }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
