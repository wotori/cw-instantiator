use crate::{
    contract::{instantiate},
    msg::InitMsg,
};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    coins, Addr,
};

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies(); // Remove the argument here
    let env = mock_env();
    let info = mock_info("creator", &coins(1000, "earth"));

    let init_msg = InitMsg {
        minter: Addr::unchecked("creator"),
        symbol: "EXAMPLE".to_string(),
        name: "Example Token".to_string(),
    };

    // Instantiate the contract
    let result = instantiate(deps.as_mut(), env, info, init_msg.clone());
    assert_eq!(result.unwrap().attributes, vec![
        ("action", "instantiate"),
        ("minter", "creator"),
        ("symbol", "EXAMPLE"),
        ("name", "Example Token"),
    ]);
}