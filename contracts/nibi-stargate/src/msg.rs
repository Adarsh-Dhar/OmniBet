use schemars::JsonSchema;
use serde::{Deserialize, Serialize};







#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub oracle : String,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum ExecuteMsg {
//     Deposit { 
//         asset: Addr, 
//         amount: u32,

//     },
//     Withdrawl { 
//         asset: Addr, 
//         amount: u32, 

//     },
//     // Borrow { 
//     //     asset: Addr, 
//     //     amount: u32,
//     //     pool_state: PoolState

//     // },
//     // Repay { 
//     //     asset: Addr, 
//     //     amount: u32,
//     //     pool_state: PoolState

//     // },
//     // Liquidate { 
//     //     borrower: String 
//     // },
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum QueryMsg {
//     User { 
//         address: String 
//     },
//     Deposit { 
//         asset: String, 
//         user: String 
//     },
//     Withdrawl { 
//         asset: String, 
//         user: String 
//     },
//     Borrow { 
//         asset: String, 
//         user: String 
//     },
//     Collateral { 
//         asset: String, 
//         user: String 
//     },
//     Liquidation { 
//         borrower: String 
//     },
//     Config {}
// }