use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, StdError
};



use crate::state::{CONFIG, POOLS, POOL_COUNT, MARKETS, MARKET_COUNT, VOTES, Pool, Market, Vote, Config};
use crate::msg::{InstantiateMsg,ExecuteMsg, QueryMsg};



pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        owner: info.sender.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    POOL_COUNT.save(deps.storage, &0u64)?;
    MARKET_COUNT.save(deps.storage, &0u64)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreatePool { token } => execute_create_pool(deps, info, token),
        ExecuteMsg::AddLiquidity { pool_id, amount } => execute_add_liquidity(deps, info, pool_id, amount),
        ExecuteMsg::CreateMarket { question, end_time } => execute_create_market(deps, info, question, end_time),
        ExecuteMsg::PlaceVote { market_id, prediction, amount } => execute_place_vote(deps, env, info, market_id, prediction, amount),
        ExecuteMsg::ResolveMarket { market_id, outcome } => execute_resolve_market(deps, info, market_id, outcome),
    }
}

fn execute_create_pool(deps: DepsMut, info: MessageInfo, token: Addr) -> StdResult<Response> {
    let mut pool_count = POOL_COUNT.load(deps.storage)?;
    pool_count += 1;
    
    let pool = Pool {
        id: pool_count,
        creator: info.sender.clone(),
        token: token.clone(),
        liquidity: 0,
    };
    
    POOLS.save(deps.storage, pool_count, &pool)?;
    POOL_COUNT.save(deps.storage, &pool_count)?;
    
    Ok(Response::new()
        .add_attribute("action", "create_pool")
        .add_attribute("pool_id", pool_count.to_string())
        .add_attribute("token", token.to_string()))
}

fn execute_add_liquidity(deps: DepsMut, info: MessageInfo, pool_id: u64, amount: i128) -> StdResult<Response> {
    let mut pool = POOLS.load(deps.storage, pool_id)?;
    
    // Check if the sent funds match the specified amount and token
    let sent_funds = info.funds.iter().find(|coin| coin.denom == pool.token.to_string()).ok_or_else(|| StdError::generic_err("No matching funds sent"))?;
    // if sent_funds.amount != amount {
    //     return Err(StdError::generic_err("Sent amount does not match specified amount"));
    // }
    
    pool.liquidity += amount;
    POOLS.save(deps.storage, pool_id, &pool)?;
    
    Ok(Response::new()
        .add_attribute("action", "add_liquidity")
        .add_attribute("pool_id", pool_id.to_string())
        .add_attribute("amount", amount.to_string()))
}

fn execute_create_market(deps: DepsMut, info: MessageInfo, question: String, end_time: u64) -> StdResult<Response> {
    let mut market_count = MARKET_COUNT.load(deps.storage)?;
    market_count += 1;
    
    let market = Market {
        id: market_count,
        question,
        end_time,
        yes_votes: 0,
        no_votes: 0,
        resolved: false,
        outcome: None,
    };
    
    MARKETS.save(deps.storage, market_count, &market)?;
    MARKET_COUNT.save(deps.storage, &market_count)?;
    
    Ok(Response::new()
        .add_attribute("action", "create_market")
        .add_attribute("market_id", market_count.to_string()))
}

fn execute_place_vote(deps: DepsMut, env: Env, info: MessageInfo, market_id: u64, prediction: bool, amount: i128) -> StdResult<Response> {
    let mut market = MARKETS.load(deps.storage, market_id)?;
    
    if market.resolved || env.block.time.seconds() >= market.end_time {
        return Err(StdError::generic_err("Market is no longer active"));
    }
    
    // Check if the sent funds match the specified amount
    let sent_funds = info.funds.iter().next().ok_or_else(|| StdError::generic_err("No funds sent"))?;
    // if sent_funds.amount != amount {
    //     return Err(StdError::generic_err("Sent amount does not match specified amount"));
    // }
    
    let vote = Vote {
        user: info.sender.clone(),
        amount,
        prediction,
    };
    
    VOTES.save(deps.storage, (market_id, info.sender.clone()), &vote)?;
    
    if prediction {
        market.yes_votes += amount;
    } else {
        market.no_votes += amount;
    }
    
    MARKETS.save(deps.storage, market_id, &market)?;
    
    Ok(Response::new()
        .add_attribute("action", "place_vote")
        .add_attribute("market_id", market_id.to_string())
        .add_attribute("prediction", prediction.to_string())
        .add_attribute("amount", amount.to_string()))
}

fn execute_resolve_market(deps: DepsMut, info: MessageInfo, market_id: u64, outcome: bool) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(StdError::generic_err("Unauthorized"));
    }
    
    let mut market = MARKETS.load(deps.storage, market_id)?;
    
    if market.resolved {
        return Err(StdError::generic_err("Market already resolved"));
    }
    
    market.resolved = true;
    market.outcome = Some(outcome);
    
    MARKETS.save(deps.storage, market_id, &market)?;
    
    Ok(Response::new()
        .add_attribute("action", "resolve_market")
        .add_attribute("market_id", market_id.to_string())
        .add_attribute("outcome", outcome.to_string()))
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetPool { id } => to_json_binary(&query_pool(deps, id)?),
        QueryMsg::GetMarket { id } => to_json_binary(&query_market(deps, id)?),
        QueryMsg::GetVote { market_id, user } => to_json_binary(&query_vote(deps, market_id, user)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

fn query_pool(deps: Deps, id: u64) -> StdResult<Pool> {
    POOLS.load(deps.storage, id)
}

fn query_market(deps: Deps, id: u64) -> StdResult<Market> {
    MARKETS.load(deps.storage, id)
}

fn query_vote(deps: Deps, market_id: u64, user: Addr) -> StdResult<Vote> {
    VOTES.load(deps.storage, (market_id, user))
}