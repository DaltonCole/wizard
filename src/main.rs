mod cards;
mod client;
mod game;
mod players;
mod server;

use crate::game::wizard::WizardGame;
use crate::server::server::Server;

fn main() {
    let mut server = Server {};
    server.start_server(4);

    //let mut game = WizardGame::new(4).unwrap();
    //game.play_game();
}
