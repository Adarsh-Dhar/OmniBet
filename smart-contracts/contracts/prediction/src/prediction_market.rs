use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use cw_storage_plus::{Map, Item};

pub const EVENT_COUNT: Item<u64> = Item::new("event_count");
pub const EVENTS: Map<u64, Event> = Map::new("events");

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub description: String,
    pub outcomes: Vec<Outcome>,
    pub total_staked: Uint128,
    pub resolved: bool,
    pub winning_outcome: Option<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    pub id: u64,
    pub description: String,
    pub total_staked: Uint128,
}

pub enum ExecuteMsg {
    CreateEvent { description: String, outcomes: Vec<String> },
    PlacePrediction { event_id: u64, outcome_id: u64, amount: Uint128 },
    ResolveEvent { event_id: u64, winning_outcome_id: u64 },
}

pub enum QueryMsg {
    GetEvent { event_id: u64 },
}

pub fn execute_create_event(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    description: String,
    outcomes: Vec<String>,
) -> StdResult<Response> {
    let mut event_count = EVENT_COUNT.load(deps.storage)?;
    event_count += 1;

    let outcomes_vec = outcomes
        .iter()
        .enumerate()
        .map(|(id, desc)| Outcome {
            id: id as u64,
            description: desc.clone(),
            total_staked: Uint128::zero(),
        })
        .collect();

    let event = Event {
        description,
        outcomes: outcomes_vec,
        total_staked: Uint128::zero(),
        resolved: false,
        winning_outcome: None,
    };

    EVENTS.save(deps.storage, event_count, &event)?;
    EVENT_COUNT.save(deps.storage, &event_count)?;

    Ok(Response::new().add_attribute("action", "create_event").add_attribute("event_id", event_count.to_string()))
}

pub fn execute_place_prediction(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    event_id: u64,
    outcome_id: u64,
    amount: Uint128,
) -> StdResult<Response> {
    let mut event = EVENTS.load(deps.storage, event_id)?;

    if event.resolved {
        return Err(StdError::generic_err("Event already resolved"));
    }

    // Simulate token transfer, adjust this part to your token transfer logic
    // Example: transfer_tokens(info.sender, amount)?;

    event.outcomes[outcome_id as usize].total_staked += amount;
    event.total_staked += amount;

    EVENTS.save(deps.storage, event_id, &event)?;

    Ok(Response::new().add_attribute("action", "place_prediction").add_attribute("event_id", event_id.to_string()))
}

pub fn execute_resolve_event(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    event_id: u64,
    winning_outcome_id: u64,
) -> StdResult<Response> {
    let mut event = EVENTS.load(deps.storage, event_id)?;

    if event.resolved {
        return Err(StdError::generic_err("Event already resolved"));
    }

    event.resolved = true;
    event.winning_outcome = Some(winning_outcome_id);

    EVENTS.save(deps.storage, event_id, &event)?;

    Ok(Response::new().add_attribute("action", "resolve_event").add_attribute("event_id", event_id.to_string()))
}

pub fn query_get_event(deps: Deps, _env: Env, event_id: u64) -> StdResult<Event> {
    let event = EVENTS.load(deps.storage, event_id)?;
    Ok(event)
}
