use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    //
    Bid,
    // "trump" and "hand" will be supplied
    ChooseTrump,
    // "msg" will be supplied detailing what we are confirming
    Confirmation,
    // "host" and "port" will be supplied
    Connect,
    // TODO - remove
    Card,
    // Final Game Stats
    EndGame,
    // "hand", "playable_cards", "played_card" will be supplied. "played_card" should be the
    // client's response
    PlayCard,
    // Tell client that server is starting the game
    StartGame,
}

impl Action {
    pub fn serde_find_action(data: &Vec<u8>) -> Result<(Action, Value), serde_json::Error> {
        let deserialized: serde_json::Value = serde_json::from_slice(data)?;
        let action = serde_json::from_value(deserialized["action"].clone())?;

        Ok((action, deserialized))
    }
}
