use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, Addr, WasmMsg, CosmosMsg,
};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    CreateMarket {
        question: String,
        end_time: u64,
        allowed_tokens: Vec<Addr>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetMarkets {},
    GetMarketsByStatus { status: MarketStatus },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum MarketStatus {
    Active,
    Resolved,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MarketInfo {
    pub id: u64,
    pub question: String,
    pub end_time: u64,
    pub status: MarketStatus,
    pub allowed_tokens: Vec<Addr>,
}

pub struct PredictionMarketFactory<'a> {
    pub markets: Map<'a, u64, MarketInfo>,
    pub market_count: Item<'a, u64>,
}

impl<'a> PredictionMarketFactory<'a> {
    pub const fn new() -> Self {
        Self {
            markets: Map::new("markets"),
            market_count: Item::new("market_count"),
        }
    }

    pub fn instantiate(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> StdResult<Response> {
        self.market_count.save(_deps.storage, &0u64)?;
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
            ExecuteMsg::CreateMarket {
                question,
                end_time,
                allowed_tokens,
            } => self.create_market(deps, env, info, question, end_time, allowed_tokens),
        }
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetMarkets {} => to_binary(&self.get_markets(deps)?),
            QueryMsg::GetMarketsByStatus { status } => {
                to_binary(&self.get_markets_by_status(deps, status)?)
            }
        }
    }

    fn create_market(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        question: String,
        end_time: u64,
        allowed_tokens: Vec<Addr>,
    ) -> StdResult<Response> {
        let id = self.market_count.load(deps.storage)? + 1;
        self.market_count.save(deps.storage, &id)?;

        let market_info = MarketInfo {
            id,
            question,
            end_time,
            status: MarketStatus::Active,
            allowed_tokens,
        };

        self.markets.save(deps.storage, id, &market_info)?;

        Ok(Response::new().add_attribute("method", "create_market").add_attribute("id", id.to_string()))
    }

    fn get_markets(&self, deps: Deps) -> StdResult<Vec<MarketInfo>> {
        self.markets
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|item| item.map(|(_, market)| market))
            .collect()
    }

    fn get_markets_by_status(&self, deps: Deps, status: MarketStatus) -> StdResult<Vec<MarketInfo>> {
        self.markets
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter(|item| {
                if let Ok((_, market)) = item {
                    market.status == status
                } else {
                    false
                }
            })
            .map(|item| item.map(|(_, market)| market))
            .collect()
    }
}