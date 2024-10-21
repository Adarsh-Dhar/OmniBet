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
            
            BET,
            BET_PREDICTION,
            BetStatus
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
        Timestamp
    },
   
    std::time::Duration,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new().add_attribute("method", "migrate"))
}

/// The instantiate function is invoked when the contract is first deployed.
/// This function sets configuration values that are used by the query function.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // It is a good practice that your contract stores the pyth contract address and ids of the
    // price feeds it needs upon instantiation or by an authorized approach. This will ensure
    // that a wrong address won't be used.
   

    Ok(Response::new()
        .add_attribute("method", "instantiate")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("method", "execute"))
}

pub fn execute_create_bet(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: ExecuteMsg,
    amount : Uint128,
    asset_name : String,
    id : Uint128,
    expiry : u64,
) -> StdResult<Response> {
    let creator = info.sender;
    let mut bet = BET.load(deps.storage)?;

    if bet.creator != creator {
        return Err(StdError::generic_err("You are not the creator of this bet"));
    }

    if bet.state != BetStatus::not_created {
        return Err(StdError::generic_err("Bet already created"));
    }

    bet.amount = amount;
    bet.asset_name = asset_name;
    

    let current_time = env.block.time;
    let expiry_time = current_time.plus_seconds(expiry); 

    bet.expiry = expiry_time;

    bet.state = BetStatus::created;


    Ok(Response::new().add_attribute("method", "execute_create_bet"))
}

pub fn execute_enter_bet(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: ExecuteMsg,
    id : Uint128,
    amount : Uint128
) -> StdResult<Response> {
    let sender = info.sender;
    
    let mut bet = BET.load(deps.storage)?;
    let mut bet_prediction = BET_PREDICTION.load(deps.storage)?;

    if bet.state != BetStatus::created {
        return Err(StdError::generic_err("Bet not created"));
    }

    bet_prediction.bet_id = id;
    bet_prediction.bet = amount;
    bet_prediction.player = sender;

    BET_PREDICTION.save(deps.storage, &bet_prediction)?;
    Ok(Response::new().add_attribute("method", "execute_enter_bet"))
}

pub fn execute_claim_bet(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: ExecuteMsg,
    id : Uint128,
) -> StdResult<Response> {
    let mut bet = BET.load(deps.storage)?;

    if bet.state != BetStatus::ended {
        return Err(StdError::generic_err("Bet hasn't ended yet"));
    }

    if bet.winner != info.sender {
        return Err(StdError::generic_err("You are not the winner"));
    }

    //transfer from escrow to winner

    bet.state = BetStatus::claimed;





    Ok(Response::new().add_attribute("method", "execute_claim_bet"))
}

pub fn execute_close_bet(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
    id : Uint128,
) -> StdResult<Response> {
    let mut bet = BET.load(deps.storage)?;

    if bet.state != BetStatus::claimed {
        return Err(StdError::generic_err("Bet hasn't been claimed yet"));
    }
    Ok(Response::new().add_attribute("method", "execute_close_bet"))
}



