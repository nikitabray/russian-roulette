use russian_roulette::{
    game::Game,
    player::Player,
    revolver::{cylinder::create_cylinder, Revolver},
};

fn main() {
    let players = vec![
        Player {
            name: String::from("Andrew"),
            ..Default::default()
        },
        Player {
            name: String::from("Michael"),
            ..Default::default()
        },
    ];
    let cylinder = create_cylinder(10);
    let mut revolver = Revolver { cylinder };
    let mut game = Game {
        players,
        revolver: &mut revolver,
    };
    game.play()
}
