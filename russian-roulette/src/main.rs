use russian_roulette::{
    game::Game,
    player::Player,
    revolver::{cylinder::Cylinder, Revolver},
    client::{LocalClient, Client},
    utils::get_player_names,
};

fn main() {
    let player_names = get_player_names();
    let mut players: Vec<Player<LocalClient>> = vec![];
    for player_name in player_names.iter() {
        players.push(Player::new(player_name, LocalClient::new()));
    }
    let cylinder = Cylinder::new(10);
    let mut revolver = Revolver::new(cylinder);
    let mut game = Game::new(players, &mut revolver);
    game.play();
    let alive_players: Vec<&Player<LocalClient>> = game.players.iter().filter(|player| player.alive).collect();
    let alive = alive_players.first();
    match alive {
        None => (),
        Some(winner) => winner
            .client
            .send_message(&format!("You won, {}", winner.name)),
    }
}
