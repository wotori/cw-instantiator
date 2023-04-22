use crate::{contract::instantiate_stored_contract, instantiate, msg::InitMsg};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    to_binary, Addr,
};

#[test]
fn test_instantiate_stored_contract() {
    let mut deps = mock_dependencies();

    let env = mock_env();
    let info = mock_info("creator", &[]);
    let init_msg = InitMsg {
        minter: Addr::unchecked("minter"),
        symbol: "SYMBOL".to_string(),
        name: "Token Name".to_string(),
    };

    // Call instantiate
    let _ = instantiate(deps.as_mut(), env, info.clone(), init_msg.clone()).unwrap();

    let code_id = 1u64;
    let init_msg = to_binary(&InitMsg {
        minter: Addr::unchecked("minter"),
        symbol: "SYMBOL".to_string(),
        name: "Token Name".to_string(),
    })
    .unwrap();
    let admin = Some(Addr::unchecked("admin"));
    let label = "test_contract".to_string();

    // Call instantiate_stored_contract
    let result = instantiate_stored_contract(
        info.clone(),
        code_id,
        init_msg.clone(),
        admin.clone(),
        label.clone(),
    )
    .unwrap();

    // Check the result
    let expected_attributes = vec![("action", "instantiate_stored_contract")];

    let msg = result
        .messages
        .into_iter()
        .next()
        .expect("Expected WasmMsg::Instantiate");

    // WasmMsg::Instantiate

    assert_eq!(
        result.attributes, expected_attributes,
        "Unexpected attributes in the response"
    );
}
