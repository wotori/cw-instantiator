use crate::{
    contract::{instantiate, instantiate_stored_contract},
    msg::InitMsg,
};
use cosmwasm_std::SubMsg;
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    to_binary, Addr, CosmosMsg, WasmMsg,
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

    let sub_msg = result.messages.into_iter().next().expect("Expected SubMsg");

    if let SubMsg {
        msg:
            CosmosMsg::Wasm(WasmMsg::Instantiate {
                code_id: _,
                msg: _,
                funds: _,
                label: _,
                admin: _,
            }),
        ..
    } = sub_msg
    {
        // expected_attributes.push(("action", "instantiate_stored_contract"));
    } else {
        panic!("Unexpected message");
    }

    assert_eq!(
        result.attributes, expected_attributes,
        "Unexpected attributes in the response"
    );
}
