use crate::{msg::ExecuteMsg, state::State, ContractError};
use cosmwasm_std::WasmMsg;
use cosmwasm_std::WasmQueryResponse;
use cosmwasm_std::{Addr, Binary, DepsMut, Env, MessageInfo, Response, StdError, StdResult};

use crate::msg::InitMsg;
use serde_json::to_vec;

pub fn instantiate_stored_contract(
    deps: DepsMut,
    info: MessageInfo,
    code_id: u64,
    init_msg: Binary,
    admin: Option<Addr>,
    label: String,
) -> Result<Response, ContractError> {
    let funds = vec![];
    let msg = WasmMsg::Instantiate {
        admin,
        code_id,
        msg: init_msg,
        funds,
        label,
    };

    let instantiate_response = deps.querier.query(&msg.into())?;
    let contract_address: Addr = match instantiate_response {
        cosmwasm_std::QueryResponse::Wasm(WasmQueryResponse::InstantiateContract {
            contract_address,
        }) => contract_address.into(),
        _ => return Err(ContractError::ContractInstantiationFailed),
    };

    let response = Response::new()
        .add_attribute("action", "instantiate_stored_contract")
        .add_attribute("contract_address", contract_address.to_string());

    Ok(response)
}

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

pub fn execute(
    deps: DepsMut,
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
        } => instantiate_stored_contract(deps, info, code_id, init_msg, admin, label),
    }
}
