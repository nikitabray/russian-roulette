use russian_roulette::{
    game::Game,
    player::Player,
    revolver::{cylinder::create_cylinder, Revolver},
    ui::notify_game_starts,
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
    let game = Game {
        players,
        revolver: &mut revolver,
    };
    notify_game_starts(&game.players);
    game.play()
}
