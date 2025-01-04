use crate::cards::card::Card;
use crate::cards::deck::Deck;
use crate::cards::normal_card::NormalCard;
use crate::cards::special_card::SpecialCard;
use crate::cards::suit::Suit;
use crate::players::player::Player;
use anyhow::{bail, Result};
use serde_json::{Map, Value};
use std::net::TcpStream;
use std::sync::mpsc;

pub struct WizardGame {
    players: Vec<Player>,
    starting_player: usize,
    round: u8,
    trump_suit: Option<Suit>,
}

impl WizardGame {
    pub fn new(
        num_players: usize,
        client_listeners: Vec<TcpStream>,
        client_writers: Vec<TcpStream>,
    ) -> Result<WizardGame> {
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
        if num_players != client_listeners.len() || num_players != client_writers.len() {
            bail!("Players does not equal number of client listeners or writers. Players: {}, Listeners: {}, Writers: {}", num_players, client_listeners.len(), client_writers.len());
        }

        let mut players = Vec::new();
        for (client_listener, client_writer) in
            client_listeners.into_iter().zip(client_writers.into_iter())
        {
            players.push(Player::new(client_listener, client_writer));
        }

        Ok(WizardGame {
            players,
            starting_player: 0,
            round: 0,
            trump_suit: None,
        })
    }

    pub fn play_game(&mut self) {
        let num_rounds = 60 / self.players.len();

        // Tell players we are starting the game
        let state = self.game_state();
        for player in self.players.iter_mut() {
            player.start_game(&state);
        }

        for _ in 0..num_rounds {
            self.perform_round().unwrap();
        }

        // Tell players the game has ended
        let state = self.game_state();
        for player in self.players.iter_mut() {
            player.end_game(&state);
        }
    }
    fn perform_round(&mut self) -> Result<()> {
        self.round += 1;
        let mut deck = Deck::new();
        // Deal the cards and set the trump suit
        self.deal(&mut deck)?;
        // Reveal trump - Allow player to choose if trump is a wizard
        self.reveal_trump(&mut deck);
        // Each player bids
        self.bid();
        // Play all cards
        self.play_cards();
        // Calculate score
        self.update_player_scores();

        Ok(())
    }

    /// Deal the shuffled deck of cards to each player based on the round number
    fn deal(&mut self, deck: &mut Deck) -> Result<()> {
        for player in self.players.iter_mut() {
            player.cards = deck.deal(self.round as usize)?;
        }

        Ok(())
    }

    /// Reveal trump. If the trump is a wizard, the player before the starting_player
    /// gets to choose the trump suit
    fn reveal_trump(&mut self, deck: &mut Deck) {
        let top_card = deck.deal(1);

        match top_card {
            Ok(card) => {
                let trump_card = &card[0];
                match trump_card {
                    Card::SpecialCard(special_card) => match special_card {
                        SpecialCard::Wizard => {
                            let dealing_player = (self.starting_player + self.players.len() - 1)
                                % self.players.len();
                            let game_state = self.game_state();
                            self.trump_suit = Some(
                                self.players
                                    .get_mut(dealing_player)
                                    .unwrap()
                                    .choose_trump(&game_state),
                            );
                        }
                        SpecialCard::Jester => {
                            self.trump_suit = None;
                        }
                    },
                    Card::NormalCard(normal_card) => {
                        self.trump_suit = Some(normal_card.suit);
                    }
                }
            }
            Err(_) => {
                self.trump_suit = None;
            }
        };
    }

    /// Let each player bid
    fn bid(&mut self) {
        for i in 0..self.players.len() {
            let players_turn = i + self.starting_player % self.players.len();
            let state = self.game_state();
            self.players[players_turn].bid(&state);
        }
    }

    /// Each player plays cards until no cards remain in their hands
    fn play_cards(&mut self) {
        // Which player is leading for this trick
        let mut leading_player = self.starting_player;

        // For each trick
        for _ in 0..self.round {
            // Keep track of what cards have been played for this trick
            let mut played_cards = Vec::new();

            // For each player
            for i in 0..self.players.len() {
                // Which player is currently playing a card
                let playing_player = (leading_player + i) % self.players.len();

                // Get game state - including which cards have been played so far and who started
                let mut state = self.game_state();
                if let Value::Object(ref mut map) = state {
                    map.insert(
                        "played_cards".to_string(),
                        serde_json::to_value(&played_cards).unwrap(),
                    );
                    map.insert(
                        "leading_player".to_string(),
                        Value::Number(playing_player.into()),
                    );
                    let leading_suit = WizardGame::leading_suit(&played_cards);
                    map.insert(
                        "leading_suit".to_string(),
                        serde_json::to_value(&leading_suit).unwrap(),
                    );
                }

                // Player plays a card
                let played_card = self.players[playing_player].play_card(&state);
                played_cards.push(played_card);
            }

            // Update taken tricks
            let winning_player = self.trick_winner(&played_cards, leading_player);
            self.players[winning_player].won_trick();
        }
    }

    /// Leading suit given a vector of cards.
    ///
    /// If a Wizard has been played, then leading suit is set to none since suit no longer matters.
    fn leading_suit(cards: &Vec<Card>) -> Option<Suit> {
        // If there are any wizards, then there is no leading suit
        if cards.contains(&Card::SpecialCard(SpecialCard::Wizard)) {
            return None;
        }

        // Return the suit of the first normal card
        for card in cards.iter() {
            if let Card::NormalCard(normal_card) = card {
                return Some(normal_card.suit);
            }
        }

        // If there are only jesters, there is no trump suit
        None
    }

    fn trick_winner(&self, cards: &Vec<Card>, leading_player: usize) -> usize {
        let mut winning_player = 0;
        let mut winning_card = &cards[0];

        let leading_suit = WizardGame::leading_suit(cards);

        for (i, new_card) in cards.iter().enumerate().skip(1) {
            if WizardGame::is_better_card(&winning_card, &new_card, leading_suit, self.trump_suit) {
                winning_player = i;
                winning_card = new_card;
            }
        }

        (winning_player + leading_player) % self.players.len()
    }

    /// Checks to see if a card is better than another card.
    ///
    /// # Arguments
    ///
    /// * `base_card` - Card to compare to
    /// * `is_better` - Card to compare
    /// * `leading_suit` - Suit that has been lead
    /// * `trump_suit` - Trump suit
    ///
    /// # Returns
    ///
    /// True if `is_better` is a better card than `base_card`
    fn is_better_card(
        base_card: &Card,
        is_better: &Card,
        leading_suit: Option<Suit>,
        trump_suit: Option<Suit>,
    ) -> bool {
        // base_card = Wizard always is better
        if let Card::SpecialCard(special_card) = base_card {
            if let SpecialCard::Wizard = special_card {
                return false;
            }
        }
        // is_better = Jester always loses
        // is_better = Wizard -> Wins if first card isn't a wizard
        if let Card::SpecialCard(special_card) = is_better {
            match special_card {
                SpecialCard::Wizard => return true,
                SpecialCard::Jester => return false,
            }
        }
        // base_card = Jester -> Loses unless is_better is a jester
        if let Card::SpecialCard(special_card) = base_card {
            if let SpecialCard::Jester = special_card {
                return true;
            }
        }

        // --- All special cards already handled. Only normal left --- //
        let base_card = match base_card {
            Card::NormalCard(normal_card) => normal_card,
            Card::SpecialCard(_) => {
                panic!("Should have handled all special card cases before this.")
            }
        };
        let is_better = match is_better {
            Card::NormalCard(normal_card) => normal_card,
            Card::SpecialCard(_) => {
                panic!("Should have handled all special card cases before this.")
            }
        };

        // Same suit, high card wins
        if base_card.suit == is_better.suit {
            return base_card.rank < is_better.rank;
        }
        // Different suit, trump suit wins first, then leading suit
        if Some(base_card.suit) == trump_suit {
            return false;
        }
        if Some(is_better.suit) == trump_suit {
            return true;
        }
        if Some(base_card.suit) == leading_suit {
            return false;
        }
        if Some(is_better.suit) == leading_suit {
            return true;
        }

        // Default to first card winning
        false
    }

    /// Update each players' scores based on the results of the round
    fn update_player_scores(&mut self) {
        for player in self.players.iter_mut() {
            player.update_score();
        }
    }

    /// Game state
    fn game_state(&self) -> Value {
        let mut state = Map::new();

        // Game state
        state.insert(
            "starting_player".to_string(),
            Value::String(format!("player-{}", self.starting_player).to_string()),
        );
        state.insert("round".to_string(), Value::Number(self.round.into()));
        state.insert(
            "trump_suit".to_string(),
            match &self.trump_suit {
                Some(suit) => serde_json::to_value(&suit).unwrap(),
                None => Value::Null,
            },
        );
        state.insert(
            "player_count".to_string(),
            Value::Number(self.players.len().into()),
        );

        // Player states
        for (i, player) in self.players.iter().enumerate() {
            let mut player_state = Map::new();

            player_state.insert("score".to_string(), Value::Number(player.score.into()));
            player_state.insert(
                "bid".to_string(),
                match player.bid {
                    Some(bid) => Value::Number(bid.into()),
                    None => Value::Null,
                },
            );
            player_state.insert(
                "tricks_taken".to_string(),
                Value::Number(player.tricks_taken.into()),
            );

            state.insert(
                format!("player-{}", i).to_string(),
                Value::Object(player_state),
            );
        }

        Value::Object(state)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::rank::Rank;
    use strum::IntoEnumIterator;

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

    #[test]
    fn play_four_player_game() {
        let mut game = WizardGame::new(4).unwrap();

        game.play_game();

        assert_eq!(15, game.round);
    }

    fn get_bid_from_state(state: &Value, player: usize) -> Option<u8> {
        let player_str = format!("player-{}", player).to_string();

        match state.get(player_str).unwrap().get("bid").unwrap() {
            Value::Number(bid) => Some(bid.as_u64().unwrap() as u8),
            Value::Null => None,
            _ => panic!("Invalid bid value!"),
        }
    }

    #[test]
    fn game_state_inital() {
        let game = WizardGame::new(4).unwrap();

        let state = game.game_state();
        println!("{}", state);

        for i in 0..game.players.len() {
            let bid = get_bid_from_state(&state, i);
            assert_eq!(None, bid);
        }
    }

    #[test]
    fn game_state_bid() {
        let mut game = WizardGame::new(4).unwrap();
        game.bid();

        let state = game.game_state();
        println!("{}", state);

        for i in 0..game.players.len() {
            let bid = get_bid_from_state(&state, i);
            assert_eq!(Some(0), bid);
        }
    }

    #[test]
    fn trick_winner() {
        let mut game = WizardGame::new(3).unwrap();

        // Wizard
        for i in 0..3 {
            assert_eq!(
                i,
                game.trick_winner(
                    &vec![
                        Card::SpecialCard(SpecialCard::Wizard),
                        Card::SpecialCard(SpecialCard::Wizard),
                        Card::SpecialCard(SpecialCard::Wizard),
                    ],
                    i
                )
            );
        }
        assert_eq!(
            0,
            game.trick_winner(
                &vec![
                    Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Ace
                    }),
                    Card::SpecialCard(SpecialCard::Wizard),
                    Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Queen
                    }),
                ],
                2
            )
        );
        // Jester
        for i in 0..3 {
            assert_eq!(
                i,
                game.trick_winner(
                    &vec![
                        Card::SpecialCard(SpecialCard::Jester),
                        Card::SpecialCard(SpecialCard::Jester),
                        Card::SpecialCard(SpecialCard::Jester),
                    ],
                    i
                )
            );
        }
        assert_eq!(
            1,
            game.trick_winner(
                &vec![
                    Card::SpecialCard(SpecialCard::Jester),
                    Card::SpecialCard(SpecialCard::Jester),
                    Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Queen
                    }),
                ],
                2
            )
        );
        // Normal cards
        assert_eq!(
            2,
            game.trick_winner(
                &vec![
                    Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Ace
                    }),
                    Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::King
                    }),
                    Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Queen
                    }),
                ],
                2
            )
        );

        game.trump_suit = Some(Suit::Heart);
        assert_eq!(
            0,
            game.trick_winner(
                &vec![
                    Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Two
                    }),
                    Card::NormalCard(NormalCard {
                        suit: Suit::Diamond,
                        rank: Rank::King
                    }),
                    Card::NormalCard(NormalCard {
                        suit: Suit::Club,
                        rank: Rank::Ace
                    }),
                ],
                0
            )
        );
        assert_eq!(
            2,
            game.trick_winner(
                &vec![
                    Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Ace
                    }),
                    Card::NormalCard(NormalCard {
                        suit: Suit::Heart,
                        rank: Rank::King
                    }),
                    Card::NormalCard(NormalCard {
                        suit: Suit::Spade,
                        rank: Rank::Queen
                    }),
                ],
                1
            )
        );
    }

    #[test]
    fn is_better_card() {
        // --- Wizards tests --- //
        assert!(!WizardGame::is_better_card(
            &Card::SpecialCard(SpecialCard::Wizard),
            &Card::SpecialCard(SpecialCard::Wizard),
            Some(Suit::Spade),
            Some(Suit::Spade)
        ));
        assert!(!WizardGame::is_better_card(
            &Card::SpecialCard(SpecialCard::Wizard),
            &Card::SpecialCard(SpecialCard::Jester),
            Some(Suit::Spade),
            Some(Suit::Spade)
        ));
        assert!(WizardGame::is_better_card(
            &Card::SpecialCard(SpecialCard::Jester),
            &Card::SpecialCard(SpecialCard::Wizard),
            Some(Suit::Spade),
            Some(Suit::Spade)
        ));

        for trump_suit in Suit::iter() {
            for lead_suit in Suit::iter() {
                for suit in Suit::iter() {
                    for rank in Rank::iter() {
                        assert!(WizardGame::is_better_card(
                            &Card::NormalCard(NormalCard { suit, rank }),
                            &Card::SpecialCard(SpecialCard::Wizard),
                            Some(trump_suit),
                            Some(lead_suit)
                        ));
                        assert!(!WizardGame::is_better_card(
                            &Card::SpecialCard(SpecialCard::Wizard),
                            &Card::NormalCard(NormalCard { suit, rank }),
                            Some(trump_suit),
                            Some(lead_suit)
                        ));
                    }
                }
            }
        }

        // --- Jester tests --- //
        assert!(!WizardGame::is_better_card(
            &Card::SpecialCard(SpecialCard::Jester),
            &Card::SpecialCard(SpecialCard::Jester),
            Some(Suit::Spade),
            Some(Suit::Spade)
        ));

        for trump_suit in Suit::iter() {
            for lead_suit in Suit::iter() {
                for suit in Suit::iter() {
                    for rank in Rank::iter() {
                        assert!(!WizardGame::is_better_card(
                            &Card::NormalCard(NormalCard { suit, rank }),
                            &Card::SpecialCard(SpecialCard::Jester),
                            Some(trump_suit),
                            Some(lead_suit)
                        ));
                        assert!(WizardGame::is_better_card(
                            &Card::SpecialCard(SpecialCard::Jester),
                            &Card::NormalCard(NormalCard { suit, rank }),
                            Some(trump_suit),
                            Some(lead_suit)
                        ));
                    }
                }
            }
        }

        // --- Normal card tests --- //
        // Trump suit wins
        assert!(WizardGame::is_better_card(
            &Card::NormalCard(NormalCard {
                suit: Suit::Heart,
                rank: Rank::Ace
            }),
            &Card::NormalCard(NormalCard {
                suit: Suit::Spade,
                rank: Rank::Two
            }),
            Some(Suit::Heart),
            Some(Suit::Spade)
        ));
        assert!(!WizardGame::is_better_card(
            &Card::NormalCard(NormalCard {
                suit: Suit::Heart,
                rank: Rank::Ace
            }),
            &Card::NormalCard(NormalCard {
                suit: Suit::Spade,
                rank: Rank::Two
            }),
            Some(Suit::Spade),
            Some(Suit::Heart),
        ));
        // Lead suit wins
        assert!(WizardGame::is_better_card(
            &Card::NormalCard(NormalCard {
                suit: Suit::Heart,
                rank: Rank::Ace
            }),
            &Card::NormalCard(NormalCard {
                suit: Suit::Spade,
                rank: Rank::Two
            }),
            Some(Suit::Spade),
            Some(Suit::Diamond)
        ));
        assert!(!WizardGame::is_better_card(
            &Card::NormalCard(NormalCard {
                suit: Suit::Heart,
                rank: Rank::Ace
            }),
            &Card::NormalCard(NormalCard {
                suit: Suit::Spade,
                rank: Rank::Two
            }),
            Some(Suit::Heart),
            Some(Suit::Diamond)
        ));
        // High card of same suit wins
        assert!(WizardGame::is_better_card(
            &Card::NormalCard(NormalCard {
                suit: Suit::Heart,
                rank: Rank::Two
            }),
            &Card::NormalCard(NormalCard {
                suit: Suit::Heart,
                rank: Rank::Ace
            }),
            Some(Suit::Club),
            Some(Suit::Diamond)
        ));
        assert!(!WizardGame::is_better_card(
            &Card::NormalCard(NormalCard {
                suit: Suit::Heart,
                rank: Rank::Ace
            }),
            &Card::NormalCard(NormalCard {
                suit: Suit::Heart,
                rank: Rank::Two
            }),
            Some(Suit::Club),
            Some(Suit::Diamond)
        ));
        // First card of none lead nor trump suit wins
        assert!(!WizardGame::is_better_card(
            &Card::NormalCard(NormalCard {
                suit: Suit::Heart,
                rank: Rank::Two
            }),
            &Card::NormalCard(NormalCard {
                suit: Suit::Spade,
                rank: Rank::Ace
            }),
            Some(Suit::Club),
            Some(Suit::Diamond)
        ));
        assert!(!WizardGame::is_better_card(
            &Card::NormalCard(NormalCard {
                suit: Suit::Heart,
                rank: Rank::Ace
            }),
            &Card::NormalCard(NormalCard {
                suit: Suit::Spade,
                rank: Rank::Two
            }),
            Some(Suit::Club),
            Some(Suit::Diamond)
        ));
    }
}
    */
