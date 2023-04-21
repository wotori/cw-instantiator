use crate::state::State;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, StdError};

use crate::msg::InitMsg;
use serde_json::to_vec;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    let state = State {
        minter: msg.minter.clone(),
        symbol: msg.symbol.clone(),
        name: msg.name.clone(),
    };

    // Save the state to the contract's storage
    deps.storage.set(
        b"state",
        &to_vec(&state)
            .map_err(|e| StdError::generic_err(format!("Serialization error: {}", e)))?,
    );

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("minter", msg.minter)
        .add_attribute("symbol", msg.symbol)
        .add_attribute("name", msg.name))
}
