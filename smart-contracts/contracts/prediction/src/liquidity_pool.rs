use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, Addr, BankMsg, Coin,
};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub allowed_tokens: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    AddLiquidity {
        token: Addr,
        amount: Uint128,
    },
    RemoveLiquidity {
        token: Addr,
        amount: Uint128,
    },
    SwapTokens {
        from_token: Addr,
        to_token: Addr,
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetPoolInfo {},
    GetUserLiquidity { user: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PoolInfo {
    pub allowed_tokens: Vec<Addr>,
    pub total_liquidity: Map<Addr, Uint128>,
}

pub struct LiquidityPool<'a> {
    pub pool_info: Item<'a, PoolInfo>,
    pub user_liquidity: Map<'a, (Addr, Addr), Uint128>,
}

impl<'a> LiquidityPool<'a> {
    pub const fn new() -> Self {
        Self {
            pool_info: Item::new("pool_info"),
            user_liquidity: Map::new("user_liquidity"),
        }
    }

    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let pool_info = PoolInfo {
            allowed_tokens: msg.allowed_tokens,
            total_liquidity: Map::new("total_liquidity"),
        };
        self.pool_info.save(deps.storage, &pool_info)?;
        Ok(Response::new().add_attribute("method", "instantiate"))
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> StdResult<Response> {
        match msg {
            ExecuteMsg::AddLiquidity { token, amount } => self.add_liquidity(deps, env, info, token, amount),
            ExecuteMsg::RemoveLiquidity { token, amount } => self.remove_liquidity(deps, env, info, token, amount),
            ExecuteMsg::SwapTokens { from_token, to_token, amount } => self.swap_tokens(deps, env, info, from_token, to_token, amount),
        }
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetPoolInfo {} => to_binary(&self.get_pool_info(deps)?),
            QueryMsg::GetUserLiquidity { user } => to_binary(&self.get_user_liquidity(deps, user)?),
        }
    }

    fn add_liquidity(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token: Addr,
        amount: Uint128,
    ) -> StdResult<Response> {
        let mut pool_info = self.pool_info.load(deps.storage)?;
        
        if !pool_info.allowed_tokens.contains(&token) {
            return Err(StdError::generic_err("Token not allowed in this pool"));
        }

        // Update total liquidity
        let mut total_liquidity = pool_info.total_liquidity.load(deps.storage, token.clone()).unwrap_or_default();
        total_liquidity += amount;
        pool_info.total_liquidity.save(deps.storage, token.clone(), &total_liquidity)?;

        // Update user liquidity
        let mut user_liquidity = self.user_liquidity.load(deps.storage, (info.sender.clone(), token.clone())).unwrap_or_default();
        user_liquidity += amount;
        self.user_liquidity.save(deps.storage, (info.sender.clone(), token.clone()), &user_liquidity)?;

        Ok(Response::new()
            .add_attribute("method", "add_liquidity")
            .add_attribute("token", token.to_string())
            .add_attribute("amount", amount.to_string()))
    }

    fn remove_liquidity(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token: Addr,
        amount: Uint128,
    ) -> StdResult<Response> {
        let mut pool_info = self.pool_info.load(deps.storage)?;
        
        if !pool_info.allowed_tokens.contains(&token) {
            return Err(StdError::generic_err("Token not allowed in this pool"));
        }

        // Check user liquidity
        let mut user_liquidity = self.user_liquidity.load(deps.storage, (info.sender.clone(), token.clone())).unwrap_or_default();
        if user_liquidity < amount {
            return Err(StdError::generic_err("Insufficient liquidity"));
        }

        // Update total liquidity
        let mut total_liquidity = pool_info.total_liquidity.load(deps.storage, token.clone()).unwrap_or_default();
        total_liquidity -= amount;
        pool_info.total_liquidity.save(deps.storage, token.clone(), &total_liquidity)?;

        // Update user liquidity
        user_liquidity -= amount;
        self.user_liquidity.save(deps.storage, (info.sender.clone(), token.clone()), &user_liquidity)?;

        // Send tokens back to user
        let msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: token.to_string(),
                amount,
            }],
        };

        Ok(Response::new()
            .add_message(msg)
            .add_attribute("method", "remove_liquidity")
            .add_attribute("token", token.to_string())
            .add_attribute("amount", amount.to_string()))
    }

    fn swap_tokens(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        from_token: Addr,
        to_token: Addr,
        amount: Uint128,
    ) -> StdResult<Response> {
        let pool_info = self.pool_info.load(deps.storage)?;
        
        if !pool_info.allowed_tokens.contains(&from_token) || !pool_info.allowed_tokens.contains(&to_token) {
            return Err(StdError::generic_err("Invalid token pair"));
        }

        // Simple constant product AMM formula: x * y = k
        let from_liquidity = pool_info.total_liquidity.load(deps.storage, from_token.clone())?;
        let to_liquidity = pool_info.total_liquidity.load(deps.storage, to_token.clone())?;

        let out_amount = (to_liquidity * amount) / (from_liquidity + amount);

        // Update liquidity pools
        pool_info.total_liquidity.save(deps.storage, from_token.clone(), &(from_liquidity + amount))?;
        pool_info.total_liquidity.save(deps.storage, to_token.clone(), &(to_liquidity - out_amount))?;

        // Send swapped tokens to user
        let msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: to_token.to_string(),
                amount: out_amount,
            }],
        };

        Ok(Response::new()
            .add_message(msg)
            .add_attribute("method", "swap_tokens")
            .add_attribute("from_token", from_token.to_string())
            .add_attribute("to_token", to_token.to_string())
            .add_attribute("in_amount", amount.to_string())
            .add_attribute("out_amount", out_amount.to_string()))
    }

    fn get_pool_info(&self, deps: Deps) -> StdResult<PoolInfo> {
        self.pool_info.load(deps.storage)
    }

    fn get_user_liquidity(&self, deps: Deps, user: Addr) -> StdResult<Vec<(Addr, Uint128)>> {
        let pool_info = self.pool_info.load(deps.storage)?;
        let mut user_liquidity = Vec::new();

        for token in pool_info.allowed_tokens {
            let amount = self.user_liquidity.load(deps.storage, (user.clone(), token.clone())).unwrap_or_default();
            user_liquidity.push((token, amount));
        }

        Ok(user_liquidity)
    }
}