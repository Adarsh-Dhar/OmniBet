
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use {
    crate::{
        msg::{
            ExecuteMsg,
            QueryMsg,
            InstantiateMsg,
            MigrateMsg,
            AllPoolsResponse
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
        Addr,
        BankMsg,
        Storage,
        from_binary,
        Decimal
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
        ExecuteMsg::CreatePool {start_date,end_date,token, amount,deadline} => execute::execute_create_pool(deps, env, info,start_date, end_date, token, amount,deadline),
        ExecuteMsg::EnterBet{ id,current_date, bet} => {
            execute::execute_enter_bet(deps, env, info, id, current_date, bet)
        },
        ExecuteMsg::ClaimBet { bet_id,current_date,real_value } => {
            execute::execute_claim_bet(deps, env, info, bet_id, current_date,real_value)
        },
    }
}

pub mod execute {
    use super::*;

    pub fn execute_create_pool(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        start_date: Uint128,
        end_date: Uint128,
        token: String,
        amount: Uint128,
        deadline: Uint128
    ) -> StdResult<Response> {
    
        let mut bets: Vec<_> = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .collect::<StdResult<_>>()?;
            
        if bets.is_empty() {
            let mut default_bet = Bet {
                id: Uint128::zero(),
                creator: info.sender.clone(),
                token: token.clone(),
                start_date: start_date,
                end_date: end_date,
                total_amount: Uint128::zero(),
                deadline: deadline,
                status : BetStatus::created
            };
            BET.save(deps.storage, &Uint128::zero().to_be_bytes(), &default_bet)?;
            return Ok(Response::new()
                .add_attribute("method", "execute_create_bet")
                .add_attribute("bet_id", default_bet.id.to_string()));
        }

        let existing_bet = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .find(|r| match r {
                Ok((_, bet)) => bet.token == token && bet.end_date == end_date && bet.deadline == deadline,
                _ => false
            });

        if existing_bet.is_some() {
            return Err(StdError::generic_err("Bet already exists for this token and date"));
        }

        // Get latest bet ID by counting existing bets and adding 2
        let bets_count = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .count();
        let next_id = Uint128::from((bets_count + 2) as u128);

        let bet = Bet {
            id: next_id,
            creator: info.sender.clone(),
            token: token.clone(),
            start_date: start_date,
            end_date: end_date,
            total_amount: Uint128::zero(),
            deadline: deadline,
            status : BetStatus::created
        };

        // Save the new bet to both maps
        BET.save(deps.storage, &next_id.to_be_bytes(), &bet)?;

        Ok(Response::new()
            .add_attribute("method", "execute_create_bet")
            .add_attribute("bet_id", bet.id.to_string()))
    }
    
    pub fn execute_enter_bet(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        id: Uint128,
        current_date : Uint128,
        bet: Decimal,
        
    ) -> StdResult<Response> {
        let amount = info.funds.iter().map(|c| c.amount).sum();
       let mut bet_struct = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .find(|r| match r {
                Ok((_, bet)) => bet.id == id,
                _ => false
            })
            .ok_or_else(|| StdError::generic_err("Bet not found"))?
            .map_err(|e| StdError::generic_err(format!("Failed to load bet: {}", e)))?
            .1;

            if current_date > bet_struct.deadline {
                return Err(StdError::generic_err("Bet deadline has reached"));
            }

        BET.update(deps.storage, &id.to_be_bytes(), |bet_opt| -> StdResult<Bet> {
            let mut bet = bet_opt.unwrap();
            bet.total_amount += amount;
            Ok(bet)
        })?;
        

        let bet_prediction = BetPrediction {
            bet_id : id,
            player : info.sender,
            bet : bet,
            amount : amount,
            reward : Uint128::zero()
        };

        BET_PREDICTION.save(deps.storage, &id.to_be_bytes(), &bet_prediction)?;
        // Return success response with attribute
        Ok(Response::new()
            .add_attribute("method", "execute_enter_bet")
            .add_attribute("bet_id", id.to_string())
            .add_attribute("amount", amount.to_string())
            
        )
    }

    fn calculate_reward(
        storage: &mut dyn Storage,
        info: MessageInfo,
        player: Addr,
        bet_id: Uint128,
        real_value: Decimal
    ) -> StdResult<Uint128> {
        let mut bet_predictions: Vec<BetPrediction> = BET_PREDICTION
            .range(storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|r| match r {
                Ok((_, bet)) => if bet.bet_id == bet_id { Some(bet) } else { None },
                _ => None
            })
            .collect();

        if bet_predictions.is_empty() {
            return Err(StdError::generic_err("No bets found for this bet_id"));
        }

        // Calculate differences and sort predictions
        let total_amount: Uint128 = bet_predictions.iter().map(|p| p.amount).sum();
        let distributable_amount = total_amount.multiply_ratio(Uint128::from(95u128), Uint128::from(100u128)); // 95% of total pool

        // Calculate and sort by difference
        bet_predictions.sort_by(|a, b| {
            let diff_a = if a.bet >= real_value { a.bet - real_value } else { real_value - a.bet };
            let diff_b = if b.bet >= real_value { b.bet - real_value } else { real_value - b.bet };
            diff_a.partial_cmp(&diff_b).unwrap()
        });

        let max_rank = bet_predictions.len();
        let rank_denominator: Uint128 = (1..=max_rank)
            .map(|i| Uint128::from(i as u128))
            .sum();

        // Constants
        let deposit_weight = Uint128::from(75u128); // 75%
        let rank_weight = Uint128::from(25u128);    // 25%

        // Calculate and save rewards
        for (index, mut prediction) in bet_predictions.iter_mut().enumerate() {
            // Calculate deposit portion
            let deposit_portion = prediction.amount
                .multiply_ratio(deposit_weight, Uint128::from(100u128))
                .multiply_ratio(distributable_amount, total_amount);

            // Calculate rank portion
            let rank_value = Uint128::from((max_rank - index) as u128);
            let rank_portion = rank_value
                .multiply_ratio(rank_weight, Uint128::from(100u128))
                .multiply_ratio(distributable_amount, rank_denominator);

            // Total reward
            prediction.reward = deposit_portion + rank_portion;

            // Save updated prediction
            BET_PREDICTION.save(storage, &bet_id.to_be_bytes(), prediction)?;

            // Return the reward for the specified player
            if prediction.player == player {
                return Ok(prediction.reward);
            }
        }

        Err(StdError::generic_err("Player not found in predictions"))
    }
    
    pub fn execute_claim_bet(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        id : Uint128,
        current_date : Uint128,
        real_value : Decimal
    ) -> StdResult<Response> {
        let player = &info.sender;
        let mut bet_struct = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .find(|r| match r {
                Ok((_, bet)) => bet.id == id,
                _ => false
            })
            .ok_or_else(|| StdError::generic_err("Bet not found"))?
            .map_err(|e| StdError::generic_err(format!("Failed to load bet: {}", e)))?
            .1;

        if current_date < bet_struct.clone().end_date {
            return Err(StdError::generic_err("Bet has not ended yet"));
        }

        let bet_id = bet_struct.clone().id;

        let mut prediction_key = bet_id.u128().to_be_bytes().to_vec();
        prediction_key.extend(player.as_bytes());
        
        let bet_prediction = BET_PREDICTION
            .load(deps.storage, &prediction_key)
            .map_err(|_| StdError::generic_err("Bet prediction not found"))?;

            let reward_amount = calculate_reward(deps.storage, info.clone(), player.clone(), bet_id, real_value)?;

            let claim_msg = BankMsg::Send {
                to_address: player.to_string(),
                amount: vec![Coin {
                    denom: bet_struct.clone().token,
                    amount: reward_amount,
                }]
            };


        
        BET.save(deps.storage, &id.to_be_bytes(), &bet_struct)?;
        Ok(Response::new()
        .add_message(claim_msg)
        .add_attribute("method", "execute_claim_bet"))
    }
    
   
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAllPool {} => to_json_binary(&query::query_get_all_pool(deps)?),
        QueryMsg::GetPoolByToken { token } => to_json_binary(&query::query_get_pool_by_token(deps, token)?),
        QueryMsg::GetPoolByDate { date } => to_json_binary(&query::query_get_pool_by_date(deps, date)?),
    }
}

pub mod query {
    use super::*;
    use crate::msg::{AllPoolsResponse, PoolsByTokenResponse, PoolsByDateResponse};

    pub fn query_get_all_pool(deps: Deps) -> StdResult<AllPoolsResponse> {
        let pools: Vec<Bet> = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .collect::<StdResult<Vec<_>>>()?
            .into_iter()
            .map(|(_, bet)| bet)
            .collect();
        Ok(AllPoolsResponse { pools })
    }

    pub fn query_get_pool_by_token(deps: Deps, token: String) -> StdResult<PoolsByTokenResponse> {
        let pools: Vec<Bet> = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| {
                item.ok().and_then(|(_, bet)| {
                    if bet.token == token {
                        Some(bet)
                    } else {
                        None
                    }
                })
            })
            .collect();
        
        Ok(PoolsByTokenResponse { pools })
    }
    
    pub fn query_get_pool_by_date(deps: Deps, deadline: Uint128) -> StdResult<PoolsByDateResponse> {
        let pools: Vec<Bet> = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| {
                item.ok().and_then(|(_, bet)| {
                    if bet.deadline == deadline {
                        Some(bet)
                    } else {
                        None
                    }
                })
            })
            .collect();
        
        Ok(PoolsByDateResponse { pools })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use crate::msg::{ExecuteMsg, QueryMsg, InstantiateMsg};

    #[test]
    fn test_create_and_query_pool() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Instantiate contract
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(Uint128::zero(), Uint128::from(res.messages.len() as u128));

        // Create a pool
        let create_msg = ExecuteMsg::CreatePool {
            start_date: Uint128::from(1000u128),
            end_date: Uint128::from(2000u128),
            token: "unibi".to_string(),
            amount: Uint128::from(1000000u128),
            deadline: Uint128::from(1500u128)
        };
        
        let info = mock_info("creator", &[]);
        let res = execute(deps.as_mut(), env.clone(), info, create_msg).unwrap();
        
        // Verify pool creation attributes
        assert_eq!(Uint128::from(2u128), Uint128::from(res.attributes.len() as u128));
        assert_eq!("method", res.attributes[0].key);
        assert_eq!("execute_create_bet", res.attributes[0].value);
        assert_eq!("bet_id", res.attributes[1].key);
        assert_eq!("0", res.attributes[1].value); // First pool should have ID 0

        // Enter bet with 500000 unibi
        let enter_msg = ExecuteMsg::EnterBet {
            id: Uint128::zero(),
            current_date: Uint128::from(1200u128),
            bet: Decimal::from_ratio(15u128, 10u128)
        };
        
        let funds = vec![Coin {
            denom: "unibi".to_string(),
            amount: Uint128::from(500000u128)
        }];
        let info = mock_info("nibi1q3anugqtcnzlt8kts7j0l08t67kfkg60kse08w", &funds);
        let res = execute(deps.as_mut(), env.clone(), info, enter_msg).unwrap();

        // Verify enter bet attributes
        assert_eq!(Uint128::from(3u128), Uint128::from(res.attributes.len() as u128));
        assert_eq!("method", res.attributes[0].key);
        assert_eq!("execute_enter_bet", res.attributes[0].value);
        assert_eq!("amount", res.attributes[2].key);
        assert_eq!("500000", res.attributes[2].value);

        // Query all pools
        let query_msg = QueryMsg::GetAllPool {};
        let res = query(deps.as_ref(), env, query_msg).unwrap();
        let pools: AllPoolsResponse = from_binary(&res).unwrap();

        // Verify query results including updated total amount
        assert_eq!(Uint128::from(1u128), Uint128::from(pools.pools.len() as u128));
        let pool = &pools.pools[0];
        assert_eq!(Uint128::zero(), pool.id);
        assert_eq!("unibi", pool.token);
        assert_eq!(Uint128::from(1000u128), pool.start_date);
        assert_eq!(Uint128::from(2000u128), pool.end_date);
        assert_eq!(Uint128::from(1500u128), pool.deadline);
        assert_eq!(BetStatus::created, pool.status);
        assert_eq!(Uint128::from(500000u128), pool.total_amount); // Verify the total amount is updated
    }
}
