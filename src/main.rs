extern crate brdgme_cmd;
extern crate thirteen_days;

use brdgme_cmd::repl;
use thirteen_days::Game;

fn main() {
    repl(&Game::default());
}
