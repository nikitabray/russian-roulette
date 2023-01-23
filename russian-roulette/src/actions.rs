use std::fmt;

pub enum Action {
    SHOOT,
    SPIN,
    UNKNOWN,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::SHOOT => write!(f, "shoot"),
            Action::SPIN => write!(f, "spin"),
            Action::UNKNOWN => write!(f, "unknown"),
        }
    }
}

pub fn get_action(action_str: &str) -> Action {
    match action_str {
        "sh" => Action::SHOOT,
        "sp" => Action::SPIN,
        _ => Action::UNKNOWN,
    }
}
