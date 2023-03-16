use std::io;

pub fn get_player_names() -> Vec<String> {
    println!("Enter player names, one by line (empty line to stop): ");
    let mut player_names: Vec<String> = vec![];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.as_str().trim() == "" {
            break
        }
        else {
            println!("Welcome to the game, {}", input);
            player_names.push(input.trim().to_string())
        }
    }
    player_names
}
