use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw_storage_plus::{Map, Item};

pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
pub const PROPOSAL_COUNT: Item<u64> = Item::new("proposal_count");

#[derive(Clone, Debug, PartialEq)]
pub struct Proposal {
    pub description: String,
    pub votes_for: Uint128,
    pub votes_against: Uint128,
    pub finalized: bool,
}

pub enum ExecuteMsg {
    CreateProposal { description: String },
    Vote { proposal_id: u64, in_favor: bool },
    FinalizeProposal { proposal_id: u64 },
}

pub fn execute_create_proposal(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    description: String,
) -> StdResult<Response> {
    let mut proposal_count = PROPOSAL_COUNT.load(deps.storage)?;
    proposal_count += 1;

    let proposal = Proposal {
        description,
        votes_for: Uint128::zero(),
        votes_against: Uint128::zero(),
        finalized: false,
    };

    PROPOSALS.save(deps.storage, proposal_count, &proposal)?;
    PROPOSAL_COUNT.save(deps.storage, &proposal_count)?;

    Ok(Response::new().add_attribute("action", "create_proposal").add_attribute("proposal_id", proposal_count.to_string()))
}

pub fn execute_vote(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    proposal_id: u64,
    in_favor: bool,
) -> StdResult<Response> {
    let mut proposal = PROPOSALS.load(deps.storage, proposal_id)?;

    if proposal.finalized {
        return Err(StdError::generic_err("Proposal already finalized"));
    }

    if in_favor {
        proposal.votes_for += Uint128::one(); // Replace with actual voting logic
    } else {
        proposal.votes_against += Uint128::one(); // Replace with actual voting logic
    }

    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new().add_attribute("action", "vote").add_attribute("proposal_id", proposal_id.to_string()))
}

pub fn execute_finalize_proposal(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    proposal_id: u64,
) -> StdResult<Response> {
    let mut proposal = PROPOSALS.load(deps.storage, proposal_id)?;

    if proposal.finalized {
        return Err(StdError::generic_err("Proposal already finalized"));
    }

    proposal.finalized = true;

    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new().add_attribute("action", "finalize_proposal").add_attribute("proposal_id", proposal_id.to_string()))
}
