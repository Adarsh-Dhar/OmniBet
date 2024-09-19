use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};


use cw_storage_plus::{Item, Map};



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: String,
    pub stablecoin: String,
    pub collateral: String,
    pub liquidation_threshold: u8,
    pub liquidation_penalty: u8,
    pub interest: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
   pub address : Addr,
   pub collateral : u32,
   pub debt : u32,
   pub health : u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub asset : Addr,
    pub amount : u32,
    pub user : Addr,
    pub time : Timestamp,
    pub interest : u8,
    pub ibTokens : u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Withdrawl {
    pub asset : Addr,
    pub amount : u32,
    pub user : Addr,
    pub time : Timestamp,
    pub interest : u8,
    pub ibTokens : u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Borrow {
    pub asset : Addr,
    pub amount : u32,
    pub user : Addr,
    pub time : Timestamp,
    pub interest : u8,
    pub repayAmount : u32,
    pub variableDebt : u32,
    pub stableDebt : u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Collateral {
    pub asset : Addr,
    pub amount : u32,
    pub user : Addr,
    pub collateralFactor : u32,
    pub inUse : bool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Liquidation {
    pub liquidator : Addr,
    pub borrower : Addr,
    pub debtAsset : Addr,
    pub collateralAsset : Addr,
    pub debtAmount : u32,
    pub collateralSeized : u32,
    pub time : u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PoolState {
    pub total_assets: u32,      // Total assets in the pool, including interest
    pub total_ib_tokens: u32,   // Total issued ibTokens
    pub stablecoin: Addr,           // The stablecoin used in the pool (e.g., USDC)
    pub interest_rate: f64,         // The current interest rate applied to borrowed assets
    pub reserve_factor: f64,        // Percentage of interest going to protocol reserves
    pub liquidation_threshold: u8,  // Liquidation threshold for collateral
    pub liquidation_penalty: u8,    // Penalty imposed on liquidation
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Interest {
    pub asset : Addr,
    pub currentRate : u32,
    pub stableRate : u32,
    pub variableRate : u32,
    pub lastUpdated : u8,
}


pub const POOLSTATE: Item<PoolState> = Item::new("pool_state");
pub const CONFIG: Item<Config> = Item::new("config");
pub const USER : Item<User> = Item::new("user");
pub const DEPOSIT : Item<Deposit> = Item::new("deposit");
pub const WITHDRAWL : Item<Withdrawl> = Item::new("withdrawl");
pub const BORROW : Item<Borrow> = Item::new("borrow");
pub const COLLATERAL : Item<Collateral> = Item::new("collateral");
pub const LIQUIDATION : Item<Liquidation> = Item::new("liquidation");
pub const INTEREST : Item<Interest> = Item::new("interest");

 
 
