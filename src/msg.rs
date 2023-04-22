use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {
    pub minter: Addr,
    pub symbol: String,
    pub name: String,
}

// Add this new enum for handling execute messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    InstantiateStoredContract {
        code_id: u64,
        init_msg: cosmwasm_std::Binary,
        admin: Option<Addr>,
        label: String,
    },
}
