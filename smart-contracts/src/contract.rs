#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use {
    crate::{
        msg::{
            ExecuteMsg,
            
            InstantiateMsg,
            MigrateMsg,

        },
        state::{
            PRIZE_DISTRIBUTION,
            BET,
            BET_PREDICTION,
            BetStatus,
            Bet,
            BetPrediction
        },
    },
    cosmwasm_std::{
        to_json_binary,
        Binary,
        Coin,
        Deps,
        DepsMut,
        Env,
        MessageInfo,
        Response,
        StdError,
        StdResult,
        Uint128,
        Timestamp,
        Addr 
    },
   
    std::time::Duration,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new().add_attribute("method", "migrate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {

    Ok(Response::new()
        .add_attribute("method", "instantiate")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreatePool {date,token, amount} => execute::execute_create_pool(deps, env, info,date, token, amount),
        ExecuteMsg::EnterBet{ id,amount, bet} => {
            execute::execute_enter_bet(deps, env, info, id, amount, bet)
        },
        // ExecuteMsg::ClaimBet { bet_id, player } => {
        //     execute::execute_claim_bet(deps, env, info, bet_id, player)
        // },
    };
    Ok(Response::new().add_attribute("method", "execute"))
}

pub mod execute {
    use super::*;

    pub fn execute_create_pool(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        date: Timestamp,
        token: String,
        amount: Uint128,
    ) -> StdResult<Response> {
        let current_time = env.block.time;
        let deadline_seconds = current_time.seconds() - date.seconds();
        let deadline = Timestamp::from_seconds(deadline_seconds);

        if BET.load(deps.storage)?.len() == 0 {
            let bet = Bet {
                id : Uint128::new(0),
                creator : info.sender,
                token : token,
                date : date,
                total_amount : Uint128::zero(),
                deadline : deadline
            };
        };

        if BET.load(&deps.storage,&token,&date).is_some() {
            return Err(StdError::generic_err("Bet already created"));
        }

        // Get latest bet ID
        let latest_bet = BET.may_load(deps.storage)?;
        let next_id = match latest_bet {
            Some(bet) => bet.id + Uint128::new(1),
            None => Uint128::new(0)
        };

        let bet = Bet {
            id: next_id,
            creator: info.sender,
            token: token,
            date: date,
            total_amount: Uint128::zero(),
            deadline: deadline
        };

        // Save the new bet
        BET.save(deps.storage, &bet)?;

        Ok(Response::new()
            .add_attribute("method", "execute_create_bet")
            .add_attribute("bet_id", next_id.to_string()))
    }
    
    pub fn execute_enter_bet(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        id: Uint128,
        amount: Uint128,
        bet : Uint128
    ) -> StdResult<Response> {
       let mut bet = BET.load(deps.storage, &id)?;

        bet.total_amount += amount;

        let bet_prediction = BetPrediction {
            bet_id : id,
            player : info.sender,
            bet : bet,
            amount : amount
        };

        BET_PREDICTION.save(deps.storage, &bet_prediction)?;
        // Return success response with attribute
        Ok(Response::new()
            .add_attribute("method", "execute_enter_bet")
            .add_attribute("bet_id", id.to_string())
            .add_attribute("amount", amount.to_string())
            
        )
    }
    
    // pub fn execute_claim_bet(
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     id : Uint128,
    //     player : Addr
    // ) -> StdResult<Response> {
    //     let mut bet = BET.load(deps.storage)?;
    
    //     if bet.bet_status != BetStatus::ended {
    //         return Err(StdError::generic_err("Bet hasn't ended yet"));
    //     }
    
    //     if bet.winner != info.sender {
    //         return Err(StdError::generic_err("You are not the winner"));
    //     }
    
    //     //transfer from escrow to winner
    
    //     bet.bet_status = BetStatus::claimed;
    
    
    
    
    
    //     Ok(Response::new().add_attribute("method", "execute_claim_bet"))
    // }
    
   
}


// #[cfg_attr(not(feature = "library"),)]
// pub fn query(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: QueryMsg,
// ) -> StdResult<Response> {
//     match msg {
//         QueryMsg::GetAllPool {} => query::query_get_all_pool(deps, env, info),
//         QueryMsg::GetPoolByToken { token } => query::query_get_pool_by_token(deps, env, info, token),
//         QueryMsg::GetPoolByDate { date } => query::query_get_pool_by_date(deps, env, info, date),
//     };
//     Ok(Response::new().add_attribute("method", "query"))
// }

// pub mod query {
//     use super::*;

//     pub fn query_get_all_pool(
//         deps: DepsMut,
//         _env: Env,
//         _info: MessageInfo,
//     ) -> StdResult<Response> {
//         let bet = BET.load(deps.storage)?;
//         Ok(Response::new().add_attribute("method", "query_get_all_pool"))
//     }

//     pub fn query_get_pool_by_token(
//         deps: DepsMut,
//         _env: Env,
//         _info: MessageInfo,
//         token : String
//     ) -> StdResult<Response> {
//         let bet = BET.load(deps.storage)?;
//         Ok(Response::new().add_attribute("method", "query_get_pool_by_token"))
//     }

//     pub fn query_get_pool_by_date(
//         deps: DepsMut,
//         _env: Env,
//         _info: MessageInfo,
//         date : Timestamp
//     ) -> StdResult<Response> {
//         let bet = BET.load(deps.storage)?;
//         Ok(Response::new().add_attribute("method", "query_get_pool_by_date"))
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{
//         mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
//     };
//     use cosmwasm_std::{coins, from_binary, OwnedDeps, Addr, coin};
//     use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
//     use crate::state::{BetStatus, Bet, BetPrediction};

//     const CREATOR: &str = "creator";
//     const PLAYER: &str = "player";
//     const DENOM: &str = "uatom";

//     fn setup_contract() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
//         let mut deps = mock_dependencies();
//         let msg = InstantiateMsg {};
//         let info = mock_info(CREATOR, &[]);
//         let env = mock_env();

//         // Instantiate the contract
//         let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         deps
//     }

    // #[test]
    // fn proper_initialization() {
    //     let mut deps = setup_contract();
        
    //     // Verify initial state
    //     let bet = BET.load(&deps.storage).unwrap();
    //     assert_eq!(bet.bet_status, BetStatus::not_created);
    // }

    // #[test]
    // fn test_create_bet() {
    //     let mut deps = setup_contract();
    //     let env = mock_env();
    //     let info = mock_info(CREATOR, &coins(100, DENOM));

    //     // Create a bet
    //     let msg = ExecuteMsg::CreatePool {
    //         owner: Addr::unchecked(CREATOR),
    //         deadline: Timestamp::from_seconds(1000u64),
    //         token: "BTC".to_string(),
    //         amount: Uint128::new(100),
    //     };

    //     let res = execute(
    //         deps.as_mut(),
    //         env.clone(),
    //         info.clone(),
    //         msg,
    //     ).unwrap();

    //     // Verify bet creation
    //     let bet = BET.load(&deps.storage).unwrap();
    //     assert_eq!(bet.creator, Addr::unchecked(CREATOR));
    //     assert_eq!(bet.bet_status, BetStatus::created);
    //     assert_eq!(bet.token, "BTC");
        
    //     // Test creating duplicate bet
    //     let msg = ExecuteMsg::CreatePool {
    //         owner: Addr::unchecked(CREATOR),
    //         deadline: Timestamp::from_seconds(1000u64),
    //         token: "ETH".to_string(),
    //         amount: Uint128::new(100),
    //     };

    //     let err = execute(
    //         deps.as_mut(),
    //         env,
    //         info,
    //         msg,
    //     ).unwrap_err();
    //     assert_eq!(err, StdError::generic_err("Bet already created"));
    // }

    // #[test]
    // fn test_enter_bet() {
    //     let mut deps = setup_contract();
    //     let env = mock_env();
        
    //     // First create a bet
    //     let create_msg = ExecuteMsg::CreatePool {
    //         owner: Addr::unchecked(CREATOR),
    //         deadline: Timestamp::from_seconds(1000u64),
    //         token: "BTC".to_string(),
    //         amount: Uint128::new(100),
    //     };
    //     let info = mock_info(CREATOR, &coins(100, DENOM));
    //     execute(deps.as_mut(), env.clone(), info, create_msg).unwrap();

    //     // Test entering bet
    //     let enter_msg = ExecuteMsg::EnterBet {
    //         id: Uint128::new(1),
    //         amount: Uint128::new(50),
    //         player: Addr::unchecked(PLAYER),
    //     };
    //     let player_info = mock_info(PLAYER, &coins(50, DENOM));
    //     let res = execute(
    //         deps.as_mut(),
    //         env.clone(),
    //         player_info,
    //         enter_msg,
    //     ).unwrap();

    //     // Verify bet prediction
    //     let prediction = BET_PREDICTION.load(&deps.storage).unwrap();
    //     assert_eq!(prediction.player, Addr::unchecked(PLAYER));
    //     assert_eq!(prediction.bet, Uint128::new(50));
    // }

    // #[test]
    // fn test_claim_bet() {
    //     let mut deps = setup_contract();
    //     let env = mock_env();
        
    //     // Setup: Create bet and enter it
    //     let create_msg = ExecuteMsg::CreatePool {
    //         owner: Addr::unchecked(CREATOR),
    //         deadline: Timestamp::from_seconds(1000u64),
    //         token: "BTC".to_string(),
    //         amount: Uint128::new(100),
    //     };
    //     execute(
    //         deps.as_mut(),
    //         env.clone(),
    //         mock_info(CREATOR, &coins(100, DENOM)),
    //         create_msg,
    //     ).unwrap();

    //     let enter_msg = ExecuteMsg::EnterBet {
    //         id: Uint128::new(1),
    //         amount: Uint128::new(50),
    //         player: Addr::unchecked(PLAYER),
    //     };
    //     execute(
    //         deps.as_mut(),
    //         env.clone(),
    //         mock_info(PLAYER, &coins(50, DENOM)),
    //         enter_msg,
    //     ).unwrap();

    //     // Manually set bet as ended and set winner
    //     let mut bet = BET.load(&deps.storage).unwrap();
    //     bet.bet_status = BetStatus::ended;
    //     bet.winner = Addr::unchecked(PLAYER);
    //     BET.save(&mut deps.storage, &bet).unwrap();

    //     // Test claiming
    //     let claim_msg = ExecuteMsg::ClaimBet {
    //         bet_id: Uint128::new(1),
    //         player: Addr::unchecked(PLAYER),
    //     };
    //     let res = execute(
    //         deps.as_mut(),
    //         env.clone(),
    //         mock_info(PLAYER, &[]),
    //         claim_msg,
    //     ).unwrap();

    //     // Verify claim
    //     let bet = BET.load(&deps.storage).unwrap();
    //     assert_eq!(bet.bet_status, BetStatus::claimed);
    // }

//     #[test]
//     fn test_queries() {
//         let mut deps = setup_contract();
//         let env = mock_env();
        
//         // Setup: Create a bet
//         let create_msg = ExecuteMsg::CreatePool {
//             owner: Addr::unchecked(CREATOR),
//             deadline: Timestamp::from_seconds(1000u64),
//             token: "BTC".to_string(),
//             amount: Uint128::new(100),
//         };
//         execute(
//             deps.as_mut(),
//             env.clone(),
//             mock_info(CREATOR, &coins(100, DENOM)),
//             create_msg,
//         ).unwrap();

//         // Test get all pools
//         let query_msg = QueryMsg::GetAllPool {};
//         let res = query(deps.as_mut(), env.clone(), mock_info(CREATOR, &[]), query_msg).unwrap();
//         // Add assertions based on expected response

//         // Test get pool by token
//         let query_msg = QueryMsg::GetPoolByToken {
//             token: "BTC".to_string(),
//         };
//         let res = query(deps.as_mut(), env.clone(), mock_info(CREATOR, &[]), query_msg).unwrap();
//         // Add assertions based on expected response

//         // Test get pool by date
//         let query_msg = QueryMsg::GetPoolByDate {
//             date: env.block.time,
//         };
//         let res = query(deps.as_mut(), env.clone(), mock_info(CREATOR, &[]), query_msg).unwrap();
//         // Add assertions based on expected response
//     }

   
// }







