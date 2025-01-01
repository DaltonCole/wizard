mod cards;
mod game;
mod players;

use crate::game::wizard::WizardGame;

fn main() {
    let mut game = WizardGame::new(4).unwrap();
    game.play_game();
}
