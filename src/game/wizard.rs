use crate::cards::card::Card;
use crate::players::player::Player;
use anyhow::{bail, Result};

pub struct WizardGame {
    players: Vec<Player>,
    starting_player: usize,
    round: u8,
}

impl WizardGame {
    pub fn new(num_players: usize) -> Result<WizardGame> {
        if num_players < 3 {
            bail!(
                "Not enough players. Minimum of 3 players required. Players requested: {}",
                num_players
            );
        } else if num_players > 6 {
            bail!(
                "Too many players. Maximum of 6 players. Players requested: {}",
                num_players
            );
        }

        let mut players = Vec::new();
        for _ in 0..num_players {
            players.push(Player::new());
        }

        Ok(WizardGame {
            players,
            starting_player: 0,
            round: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_enough_players() {
        assert!(WizardGame::new(2).is_err());
        assert!(WizardGame::new(1).is_err());
        assert!(WizardGame::new(0).is_err());
    }

    #[test]
    fn too_many_players() {
        assert!(WizardGame::new(7).is_err());
        assert!(WizardGame::new(8).is_err());
    }

    #[test]
    fn enough_players() {
        for i in 3..=6 {
            assert!(!WizardGame::new(i).is_err());
        }
    }

    #[test]
    fn player_creation() {
        for num_players in 3..=6 {
            let mut game = WizardGame::new(num_players).unwrap();

            assert_eq!(num_players, game.players.len());

            for player in game.players {
                assert_eq!(0, player.score);
                assert_eq!(0, player.cards.len());
            }
        }
    }
}
