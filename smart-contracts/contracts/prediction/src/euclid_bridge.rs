use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, Addr, WasmMsg, CosmosMsg, SubMsg,
};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub euclid_vsl_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    InitiateSwap {
        from_token: Addr,
        to_token: Addr,
        amount: Uint128,
        min_amount_out: Uint128,
    },
    FinalizeSwap {
        swap_id: String,
        recipient: Addr,
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetSwapStatus { swap_id: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum SwapStatus {
    Pending,
    Completed,
    Failed,
}

pub struct EuclidBridge<'a> {
    pub euclid_vsl_address: Item<'a, Addr>,
    pub swaps: Map<'a, String, SwapStatus>,
}

impl<'a> EuclidBridge<'a> {
    pub const fn new() -> Self {
        Self {
            euclid_vsl_address: Item::new("euclid_vsl_address"),
            swaps: Map::new("swaps"),
        }
    }

    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        self.euclid_vsl_address.save(deps.storage, &msg.euclid_vsl_address)?;
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
            ExecuteMsg::InitiateSwap {
                from_token,
                to_token,
                amount,
                min_amount_out,
            } => self.initiate_swap(deps, env, info, from_token, to_token, amount, min_amount_out),
            ExecuteMsg::FinalizeSwap {
                swap_id,
                recipient,
                amount,
            } => self.finalize_swap(deps, env, info, swap_id, recipient, amount),
        }
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetSwapStatus { swap_id } => to_binary(&self.get_swap_status(deps, swap_id)?),
        }
    }

    fn initiate_swap(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        from_token: Addr,
        to_token: Addr,
        amount: Uint128,
        min_amount_out: Uint128,
    ) -> StdResult<Response> {
        // Generate a unique swap ID
        let swap_id = format!("swap_{}_{}", info.sender, _env.block.height);

        // Save the swap status
        self.swaps.save(deps.storage, swap_id.clone(), &SwapStatus::Pending)?;

        // Prepare the message to send to Euclid VSL
        let vsl_msg = WasmMsg::Execute {
            contract_addr: self.euclid_vsl_address.load(deps.storage)?.to_string(),
            msg: to_binary(&EuclidVSLMsg::InitiateSwap {
                from_token,
                to_token,
                amount,
                min_amount_out,
                recipient: info.sender,
                swap_id: swap_id.clone(),
            })?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_message(vsl_msg)
            .add_attribute("method", "initiate_swap")
            .add_attribute("swap_id", swap_id))
    }

    fn finalize_swap(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        swap_id: String,
        recipient: Addr,
        amount: Uint128,
    ) -> StdResult<Response> {
        // Ensure only the Euclid VSL can finalize swaps
        if info.sender != self.euclid_vsl_address.load(deps.storage)? {
            return Err(StdError::generic_err("Unauthorized"));
        }

        // Update swap status
        self.swaps.save(deps.storage, swap_id.clone(), &SwapStatus::Completed)?;

        // Transfer tokens to the recipient
        let transfer_msg = BankMsg::Send {
            to_address: recipient.to_string(),
            amount: vec![Coin {
                denom: "unibi".to_string(), // Assuming Nibiru's native token
                amount,
            }],
        };

        Ok(Response::new()
            .add_message(transfer_msg)
            .add_attribute("method", "finalize_swap")
            .add_attribute("swap_id", swap_id)
            .add_attribute("recipient", recipient)
            .add_attribute("amount", amount))
    }

    fn get_swap_status(&self, deps: Deps, swap_id: String) -> StdResult<SwapStatus> {
        self.swaps.load(deps.storage, swap_id)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum EuclidVSLMsg {
    InitiateSwap {
        from_token: Addr,
        to_token: Addr,
        amount: Uint128,
        min_amount_out: Uint128,
        recipient: Addr,
        swap_id: String,
    },
}