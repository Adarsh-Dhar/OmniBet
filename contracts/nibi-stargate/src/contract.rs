#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
// use cw2::set_contract_version;

use crate::state::{Config, CONFIG};

use crate::error::ContractError;
use crate::msg::{InstantiateMsg};




// version info for migration info
// const CONTRACT_NAME: &str = "crates.io:backend-contract";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let oracle = msg.oracle.to_string();
    let validated_oracle = deps.api.addr_validate(&oracle)?;
    

    let config = Config {
        oracle : validated_oracle.clone(),
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
    .add_attribute("action", "instantiate")
    .add_attribute("oracle", validated_oracle.to_string())
    
    )

}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn execute(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> Result<Response, ContractError> {
//     set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

//     match msg {
//         ExecuteMsg::Deposit {
//             asset,
//             amount,

//         } => execute_deposit(deps, env, info, asset, amount),
//         ExecuteMsg::Withdrawl {
//             asset,
//             amount,

//         } => execute_withdrawl(deps, env, info, asset, amount),
//     }
// }

// fn execute_deposit(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     asset: Addr,
//     amount: u32,

// ) -> Result<Response, ContractError> {
//     let user = info.sender.to_string();
//     let validated_user = deps.api.addr_validate(&user)?;

//     let mut pool_state = POOLSTATE.load(deps.storage)?;

//     let config = CONFIG.load(deps.storage)?;


//     let exchange_rate =  if pool_state.total_ib_tokens == 0 {
//         1.0 // Initial exchifange rate when no tokens have been issued
//     } else {
//         pool_state.total_assets as f32 / pool_state.total_ib_tokens as f32
//     };

//     let ib_tokens_to_mint = (amount as f32 / exchange_rate) as u32;

//     // Update the pool state with the new deposit
//     pool_state.total_assets += amount as u32; // Add the deposit to total assets
//     pool_state.total_ib_tokens += ib_tokens_to_mint as u32; // Mint ibTokens

//     POOLSTATE.save(deps.storage, &pool_state)?;

//     let deposit = Deposit {
//         asset: asset.clone(),
//         amount,
//         user: validated_user.clone(),
//         time: env.block.time,
//         interest : config.interest,
//         ibTokens : ib_tokens_to_mint
//     };

//     DEPOSIT.save(deps.storage, &deposit)?;
    

//     Ok(Response::new()
//     .add_attribute("action", "deposit")
//     .add_attribute("asset", asset)
//     .add_attribute("amount", amount.to_string())
//     .add_attribute("user", validated_user.to_string())
//     .add_attribute("time", env.block.time.to_string())
//     .add_attribute("interest", config.interest.to_string())
//     .add_attribute("ib_tokens", ib_tokens_to_mint.to_string())
//     )
// }


// fn execute_withdrawl(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     asset: Addr,
//     amount: u32

// ) -> Result<Response, ContractError> {
//     let user = info.sender.to_string();
//     let validated_user = deps.api.addr_validate(&user)?;
//     let mut pool_state = POOLSTATE.load(deps.storage)?;



//     let exchange_rate =  if pool_state.total_ib_tokens == 0 {
//         1.0 // Initial exchifange rate when no tokens have been issued
//     } else {
//         pool_state.total_assets as f32 / pool_state.total_ib_tokens as f32
//     };

//     let config = CONFIG.load(deps.storage)?;


//     let ib_tokens_to_burn = (amount as f32 / exchange_rate) as u32;

//     // Update the pool state with the new withdrawl
//     pool_state.total_assets -= amount as u32; // Subtract the withdrawl from total assets
//     pool_state.total_ib_tokens -= ib_tokens_to_burn as u32; // Burn ibTokens

//     POOLSTATE.save(deps.storage, &pool_state)?;

//     let withdrawl = Withdrawl {
//         asset: asset.clone(),
//         amount,
//         user: validated_user.clone(),
//         time: env.block.time,
//         interest : config.interest,
//         ibTokens : ib_tokens_to_burn
//     };

//     WITHDRAWL.save(deps.storage, &withdrawl)?;
    

//     Ok(Response::new()
//     .add_attribute("action", "withdrawl")
//     .add_attribute("asset", asset)
//     .add_attribute("amount", amount.to_string())
//     .add_attribute("user", validated_user.to_string())
//     .add_attribute("time", env.block.time.to_string())
//     .add_attribute("interest", config.interest.to_string())
//     .add_attribute("ib_tokens", ib_tokens_to_burn.to_string())
//     )
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use crate::contract::{instantiate};
    use crate::msg::{
        InstantiateMsg
    };
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary};

    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";



    #[test]
    fn test_instantiate_with_oracle() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        // Send as ADDR1 to show oracle is different
        let info = mock_info(ADDR1, &[]);

        // Create a message where ADDR2 will be an oracle
        // Have to use .to_string() method
        let msg = InstantiateMsg {
            oracle: ADDR2.to_string(),
        };
        // Unwrap to assert success
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        // Assert oracle is ADDR2 instead
        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("oracle", ADDR2),]
        );
    }
    
    // fn test_execute_deposit() {
    //     let mut deps = mock_dependencies();
    //     let env = mock_env();
    //     let info = mock_info(oracle, &[]);
    //     let COLLATERAL: Addr = Addr::unchecked("collateral");
    //     let msg = InstantiateMsg {
    //            oracle : oracle.to_string(),
    //             stablecoin : STABLECOIN.to_string(),
    //             collateral : COLLATERAL.to_string(),
    //             liquidation_threshold : LIQUIDATION_THRESHOLD,
    //             liquidation_penalty : LIQUIDATION_PENALTY,
    //             interest : INTEREST,
    //     };
        
        
    //     let res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

    //     let msg = ExecuteMsg::Deposit {
    //         asset : COLLATERAL,
    //         amount : 100,
    //     };
    // }

    // fn test_execute_withdrawl() {
    //     let mut deps = mock_dependencies();
    //     let env = mock_env();
    //     let info = mock_info(oracle, &[]);
    //     let COLLATERAL: Addr = Addr::unchecked("collateral");
    //     let msg = InstantiateMsg {
    //            oracle : oracle.to_string(),
    //             stablecoin : STABLECOIN.to_string(),
    //             collateral : COLLATERAL.to_string(),
    //             liquidation_threshold : LIQUIDATION_THRESHOLD,
    //             liquidation_penalty : LIQUIDATION_PENALTY,
    //             interest : INTEREST,
    //     };
        
        
    //     let res = instantiate(deps.as_mut(), env, info.clone(), msg).unwrap();

    //     let msg = ExecuteMsg::Deposit {
    //         asset : COLLATERAL,
    //         amount : 100,
    //     };
    // }
}