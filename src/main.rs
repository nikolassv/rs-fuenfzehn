mod game;
mod pattern;

use termion::raw::IntoRawMode;

use std::env;
use std::io;

const SIZE_ARG_ERROR: &str = "Please define a size for the board as a valid number from 2 to 10.";

const HELP: &str = r#"
Game of Fifteen

Arrange the tiles in ascending order by moving them to the open position. Press 'q' to give up and leave the game.

You can change the size of the board by giving a single argument to the program like:

  rs-fuenfzehn 6

The size must be between 2 and 10.
"#;

fn main() {
    let mut size = 4;

    let args: Vec<String> = env::args().collect();

    if let Some(arg) = args.get(1) {
        if arg == "help" || arg == "h" || arg == "-h" || arg == "--help" {
            println!("{}", HELP);
            return;
        }

        match arg.parse::<u8>() {
            Ok(n) if n > 1 && n < 11 => size = n,
            _ => {
                println!("{}", SIZE_ARG_ERROR);
                return;
            }
        }
    }

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = io::stdin().lock();

    let mut g = game::Game::new(size as usize, &mut stdin, &mut stdout);
    g.start();
}
