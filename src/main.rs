use std::env;
use sudoku::Game;

fn main() {
    let game_string = env::args()
        .nth(1)
        .expect("Must call program with game in string form");
    match Game::parse(&game_string) {
        Some(mut game) => {
            println!("Before:");
            println!("{:?}", game);
            game.solve();
            println!("After:");
            println!("{:?}", game);
        }
        None => println!("The puzzle failed to parse. Please give a valid puzzle"),
    }
}
