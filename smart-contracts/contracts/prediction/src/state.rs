use cosmwasm_std::{Addr, Uint128, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use pyth_sdk_cw::{Price, PriceIdentifier};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,JsonSchema)]
pub struct Bet {
    pub id: Uint128,
    pub state: BetState,
    pub price_key: String,
    pub expiry: Timestamp,
}

pub struct Predictions {
    pub predictions : Map<Addr, String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,JsonSchema)]
pub struct BetState {
    pub created: bool,
    pub started: bool, 
    pub player_won : Addr,
    pub draw : bool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,JsonSchema)]
pub struct BetPrediction {
    pub player: Addr,
    pub price : Price,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Oracle {
    // Available price feeds and their ids are listed in pyth-sdk-cw Readme.
    pub price_feed_id:  PriceIdentifier,
    // Contract address of Pyth in different networks are listed in pyth-sdk-cw Readme.
    pub pyth_contract_addr: Addr,
}

pub const ORACLE: Item<Oracle> = Item::new("oracle");

pub const BET: Item<Bet> = Item::new("bet");
pub const BET_STATE : Item<BetState> = Item::new("bet_state");
pub const BET_PREDICTION : Item<BetPrediction> = Item::new("bet_prediction");



