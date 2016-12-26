#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate brdgme_game;
extern crate brdgme_color;
extern crate brdgme_markup;

use brdgme_game::{Gamer, GameError, Log};

mod render;

#[derive(Serialize, Deserialize)]
pub struct PubState {

}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Game {
}

impl Gamer for Game {
    type PubState = PubState;

    fn start(&mut self, players: usize) -> Result<Vec<Log>, GameError> {
        Err(GameError::Internal("not implemented".to_string()))
    }

    fn is_finished(&self) -> bool {
        false
    }

    fn winners(&self) -> Vec<usize> {
        vec![]
    }

    fn whose_turn(&self) -> Vec<usize> {
        match self.is_finished() {
            false => vec![0],
            true => vec![],
        }
    }

    fn pub_state(&self, player: Option<usize>) -> Self::PubState {
        PubState {}
    }

    fn command(&mut self,
               player: usize,
               input: &str,
               players: &[String])
               -> Result<(Vec<Log>, String), GameError> {
        Err(GameError::Internal("not implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
