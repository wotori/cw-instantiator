pub mod contract;
mod error;
pub mod msg;
pub mod state;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, entry_point};
use msg::{ExecuteMsg, InitMsg};
use std::result::Result; // Add this line

pub use crate::error::ContractError;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    contract::execute(deps, env, info, msg)
}

#[cfg(test)]
mod tests;
