pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);

#[cfg(test)]
mod tests; 