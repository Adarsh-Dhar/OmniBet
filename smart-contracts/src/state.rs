use cosmwasm_std::{Addr, Uint128, Timestamp, Decimal};
use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

use cw_storage_plus::{Item, Map};



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BetPrediction {
    pub player: Addr,
    pub bet: Decimal,
    pub bet_id : Uint128,
    pub amount : Uint128,
    pub reward : Uint128,
}





#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bet {
    pub id : Uint128,
    pub creator : Addr,
    pub token : String,
    pub start_date : Uint128,
    pub end_date : Uint128,
    pub total_amount : Uint128,
    pub deadline : Uint128,
    pub status : BetStatus,
}



pub struct Bets<'a> {
    pub bet : Map<'a, String, Uint128>
}

pub struct Prediction<'a> {
    pub prediction : Map<'a, String, BetPrediction>
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum BetStatus {
    vote,
    claim
}


pub const BET: Map<&[u8], Bet> = Map::new("bet");
pub const BET_PREDICTION: Map<&[u8], BetPrediction> = Map::new("bet_prediction");
pub const BETS: Map<&str, Bet> = Map::new("bets");
pub const PRIZE_DISTRIBUTION: Item<Vec<Uint128>> = Item::new("prize_distribution");