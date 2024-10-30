use cosmwasm_std::{Addr, Uint128, Timestamp};
use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

use cw_storage_plus::{Item, Map};



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BetPrediction {
    pub player: Addr,
    pub bet: Uint128,
    pub bet_id : Uint128,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bet {
    pub id : Uint128,
    pub creator : Addr,
    pub amount: Uint128,
    pub bet_status : BetStatus,
    pub expiry : Timestamp,
    pub token : String,
    pub winner : Addr
}



pub struct Bets<'a> {
    pub bet : Map<'a, String, Uint128>
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


pub const BET: Item<Bet> = Item::new("bet");
pub const BET_PREDICTION: Item<BetPrediction> = Item::new("bet_prediction");
pub const BETS: Map<&str, Bet> = Map::new("bets");
pub const PRIZE_DISTRIBUTION: Item<Vec<Uint128>> = Item::new("prize_distribution");


