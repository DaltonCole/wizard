use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    // "msg" will be supplied detailing what we are confirming
    Confirmation,
    // "port" will be supplied
    Connect,
    //
    RequestBid,
    //
    GiveBid,
    // TODO - remove
    Card,
}

impl Action {
    pub fn serde_find_action(data: &Vec<u8>) -> Result<(Action, Value), serde_json::Error> {
        let deserialized: serde_json::Value = serde_json::from_slice(data)?;
        let action = serde_json::from_value(deserialized["action"].clone())?;

        Ok((action, deserialized))
    }
}
