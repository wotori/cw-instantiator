use crate::{msg::ExecuteMsg, state::State, ContractError};
use cosmwasm_std::{Addr, Binary, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{StdError, WasmMsg};

use crate::msg::InitMsg;
use serde_json::to_vec;

pub fn instantiate_stored_contract(
    _info: MessageInfo,
    code_id: u64,
    init_msg: Binary,
    admin: Option<Addr>,
    label: String,
) -> Result<Response, ContractError> {
    let funds = vec![];
    let msg = WasmMsg::Instantiate {
        admin: admin.map(|addr| addr.to_string()),
        code_id,
        msg: init_msg,
        funds,
        label,
    };

    Ok(Response::new()
        .add_attribute("hello", "world")
        .add_attribute("action", "instantiate_stored_contract")
        .add_message(msg))
}

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    let state = State {
        minter: msg.minter.clone(),
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
        .add_attribute("name", msg.name))
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InstantiateStoredContract {
            code_id,
            init_msg,
            admin,
            label,
        } => instantiate_stored_contract(info, code_id, init_msg, admin, label),
    }
}
