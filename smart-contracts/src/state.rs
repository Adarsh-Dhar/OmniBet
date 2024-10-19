use cosmwasm_std::{Addr, Uint128, Timestamp};
use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

use cw_storage_plus::{Item, Map};
use pyth_sdk_cw::{PriceIdentifier, Price};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    // Available price feeds and their ids are listed in pyth-sdk-cw Readme.
    pub price_feed_id: PriceIdentifier,
    // Contract address of Pyth in different networks are listed in pyth-sdk-cw Readme.
    pub pyth_contract_addr: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BetPrediction {
    pub player: Addr,
    pub bet: Price,
    pub bet_id : Uint128,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bet {
    pub id : Uint128,
    pub creator : Addr,
    pub amount: Uint128,
    pub state : BetStatus,
    pub expiry : Timestamp,
    pub asset_name : String,
    pub winner : Addr,
}

pub struct Prediction<'a> {
    pub prediction : Map<'a, String, BetPrediction>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum BetStatus {
    not_created,
    created,
    ended,
    claimed,
}

pub const STATE: Item<State> = Item::new("state");
pub const BET: Item<Bet> = Item::new("bet");
pub const BET_PREDICTION: Item<BetPrediction> = Item::new("bet_prediction");


