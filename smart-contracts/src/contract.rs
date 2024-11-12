
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use {
    crate::{
        msg::{
            ExecuteMsg,
            QueryMsg,
            InstantiateMsg,
            MigrateMsg,

        },
        state::{

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

        Addr,
        BankMsg,
        Storage,

        Decimal
    },
   

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
                status : BetStatus::vote
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
            status : BetStatus::vote
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
        
        // Check if player has already entered this bet
        let existing_prediction = BET_PREDICTION
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .find(|r| match r {
                Ok((_, pred)) => pred.bet_id == id && pred.player == info.sender,
                _ => false
            });

        if existing_prediction.is_some() {
            return Err(StdError::generic_err("Player has already entered this bet"));
        }

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
        // Get all predictions for this bet_id
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

        let total_participants = bet_predictions.len();
        let total_amount: Uint128 = bet_predictions.iter().map(|p| p.amount).sum();
        let distributable_amount = total_amount.multiply_ratio(Uint128::from(95u128), Uint128::from(100u128));

        // Calculate differences and store with index
        let mut predictions_with_diff: Vec<(usize, Decimal, &mut BetPrediction)> = bet_predictions
            .iter_mut()
            .enumerate()
            .map(|(i, pred)| {
                let diff = if pred.bet >= real_value { 
                    pred.bet - real_value 
                } else { 
                    real_value - pred.bet 
                };
                (i, diff, pred)
            })
            .collect();

        // Sort by difference
        predictions_with_diff.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Assign ranks (same diff = same rank)
        let mut current_rank = 1;
        let mut prev_diff = predictions_with_diff[0].1;
        let mut rank_map: Vec<(usize, u32)> = Vec::new();

        for (i, diff, _) in predictions_with_diff.iter() {
            if *diff > prev_diff {
                current_rank += 1;
                prev_diff = *diff;
            }
            rank_map.push((*i, current_rank));
        }

        // Sort back to original order
        rank_map.sort_by_key(|k| k.0);

        // Calculate rewards based on rank
        for (i, (_, rank)) in rank_map.iter().enumerate() {
            let mut prediction = &mut bet_predictions[i];
            
            // Calculate deposit portion (75%)
            let deposit_portion = prediction.amount
                .multiply_ratio(Uint128::from(75u128), Uint128::from(100u128))
                .multiply_ratio(distributable_amount, total_amount);

            // Calculate rank portion (25%)
            let rank_portion = Uint128::from((total_participants + 1 - *rank as usize) as u128)
                .multiply_ratio(Uint128::from(25u128), Uint128::from(100u128))
                .multiply_ratio(distributable_amount, Uint128::from(total_participants as u128));

            prediction.reward = deposit_portion + rank_portion;
            
            // Save updated prediction
            BET_PREDICTION.save(storage, &bet_id.to_be_bytes(), prediction)?;

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
        QueryMsg::GetAllPool {current_time} => to_json_binary(&query::query_get_all_pool(deps, current_time)?),
        QueryMsg::GetPoolByToken { token, current_time } => to_json_binary(&query::query_get_pool_by_token(deps, token, current_time)?),
        QueryMsg::GetPoolByDate { date, current_time } => to_json_binary(&query::query_get_pool_by_date(deps, date, current_time)?),
    }
}

pub mod query {
    use super::*;
    use crate::msg::{AllPoolsResponse, PoolsByTokenResponse, PoolsByDateResponse};

    pub fn query_get_all_pool(deps: Deps, current_time: Uint128) -> StdResult<AllPoolsResponse> {
        let pools: Vec<Bet> = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .collect::<StdResult<Vec<_>>>()?
            .into_iter()
            .map(|(_, mut bet)| {
                if current_time < bet.deadline {
                    bet.status = BetStatus::vote;
                }else{
                    bet.status = BetStatus::claim;
                }
                bet
            })
            .collect();
        Ok(AllPoolsResponse { pools })
    }

    pub fn query_get_pool_by_token(deps: Deps, token: String, current_time: Uint128) -> StdResult<PoolsByTokenResponse> {
        let pools: Vec<Bet> = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| {
                item.ok().and_then(|(_, mut bet)| {
                    if bet.token == token {
                        if current_time < bet.deadline {
                            bet.status = BetStatus::vote;
                        }else{
                            bet.status = BetStatus::claim;
                        }
                        Some(bet)
                    } else {
                        None
                    }
                })
            })
            .collect();
        
        Ok(PoolsByTokenResponse { pools })
    }
    
    pub fn query_get_pool_by_date(deps: Deps, deadline: Uint128, current_time: Uint128) -> StdResult<PoolsByDateResponse> {
        let pools: Vec<Bet> = BET
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| {
                item.ok().and_then(|(_, mut bet)| {
                    if bet.deadline == deadline {
                        if current_time < bet.deadline {
                            bet.status = BetStatus::vote;
                        }else{
                            bet.status = BetStatus::claim;
                        }
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

    fn setup_test_pools(deps: &mut cosmwasm_std::OwnedDeps<cosmwasm_std::MemoryStorage, cosmwasm_std::testing::MockApi, cosmwasm_std::testing::MockQuerier>) -> StdResult<()> {
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Instantiate contract
        let msg = InstantiateMsg {};
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg)?;

        // Create multiple test pools
        let pools = vec![
            (
                "unibi".to_string(),
                Uint128::from(1000u128),
                Uint128::from(2000u128),
                Uint128::from(1500u128)
            ),
            (
                "atom".to_string(), 
                Uint128::from(1500u128),
                Uint128::from(2500u128),
                Uint128::from(2000u128)
            ),
            (
                "osmo".to_string(),
                Uint128::from(1800u128),
                Uint128::from(2800u128),
                Uint128::from(2300u128)
            ),
            (
                "unibi".to_string(), // Second unibi pool
                Uint128::from(2000u128),
                Uint128::from(3000u128),
                Uint128::from(2500u128)
            )
        ];

        for (token, start_date, end_date, deadline) in pools {
            let create_msg = ExecuteMsg::CreatePool {
                start_date,
                end_date,
                token,
                amount: Uint128::from(1000000u128),
                deadline
            };
            execute(deps.as_mut(), env.clone(), info.clone(), create_msg)?;
        }
        
        Ok(())
    }

    #[test]
    fn test_create_and_query_pool() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Instantiate contract
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

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
        assert_eq!(2, res.attributes.len());
        assert_eq!("method", res.attributes[0].key);
        assert_eq!("execute_create_bet", res.attributes[0].value);
        assert_eq!("bet_id", res.attributes[1].key);
        assert_eq!("0", res.attributes[1].value);

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
        let info = mock_info("player", &funds);
        let res = execute(deps.as_mut(), env.clone(), info, enter_msg).unwrap();

        assert_eq!(3, res.attributes.len());
        assert_eq!("method", res.attributes[0].key);
        assert_eq!("execute_enter_bet", res.attributes[0].value);
        assert_eq!("amount", res.attributes[2].key);
        assert_eq!("500000", res.attributes[2].value);
    }

    #[test]
    fn test_query_all_pools() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        setup_test_pools(&mut deps).unwrap();

        // Test pools before end date
        let query_msg = QueryMsg::GetAllPool {};
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let pools: AllPoolsResponse = from_binary(&res).unwrap();
        
        assert_eq!(4, pools.pools.len());
        for pool in pools.pools {
            assert_eq!(BetStatus::created, pool.status);
        }

        // Test pools after some end dates
        let query_msg = QueryMsg::GetAllPool {};
        let current_time = Uint128::from(2300u128); // After first unibi and atom pools end
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let pools: AllPoolsResponse = from_binary(&res).unwrap();
        
        for pool in pools.pools {
            if pool.end_date < current_time {
                assert_eq!(BetStatus::Ended, pool.status);
            } else {
                assert_eq!(BetStatus::created, pool.status);
            }
        }
    }

    #[test]
    fn test_query_pools_by_token() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        setup_test_pools(&mut deps).unwrap();

        // Query unibi pools
        let query_msg = QueryMsg::GetPoolByToken { 
            token: "unibi".to_string() 
        };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let pools: PoolsByTokenResponse = from_binary(&res).unwrap();
        
        assert_eq!(2, pools.pools.len());
        for pool in pools.pools {
            assert_eq!("unibi", pool.token);
        }

        // Query atom pools
        let query_msg = QueryMsg::GetPoolByToken { 
            token: "atom".to_string() 
        };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let pools: PoolsByTokenResponse = from_binary(&res).unwrap();
        
        assert_eq!(1, pools.pools.len());
        assert_eq!("atom", pools.pools[0].token);
    }

    #[test]
    fn test_query_pools_by_date() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        setup_test_pools(&mut deps).unwrap();

        // Query pools by specific deadline
        let query_msg = QueryMsg::GetPoolByDate { 
            date: Uint128::from(2000u128)  // Deadline for atom pool
        };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let pools: PoolsByDateResponse = from_binary(&res).unwrap();
        
        assert_eq!(1, pools.pools.len());
        assert_eq!(Uint128::from(2000u128), pools.pools[0].deadline);
        assert_eq!("atom", pools.pools[0].token);

        // Query pools by another deadline
        let query_msg = QueryMsg::GetPoolByDate { 
            date: Uint128::from(2500u128)  // Deadline for second unibi pool
        };
        let res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let pools: PoolsByDateResponse = from_binary(&res).unwrap();
        
        assert_eq!(1, pools.pools.len());
        assert_eq!(Uint128::from(2500u128), pools.pools[0].deadline);
        assert_eq!("unibi", pools.pools[0].token);
    }
}