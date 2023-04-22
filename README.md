# Instantiator

The Instantiator contract was developed during the DoraHacks hackathon for Archway Network and the NFText dApp. This contract allows users to instantiate new instances of smart contracts using the already stored .wasm file. The main purpose of this contract is to enable users to instantiate CW721 contracts for their art collections that can be used on NFT platforms.
## Workflow

1. Create the base64 JSON file with data for contract instantiation and pass it as an `init_msg` parameter.

2. Build the contract using the `archway build --optimize` command.

3. Instantiate the contract using the `archway instantiate` command, passing in the necessary parameters including the `code_id` of the stored .wasm file.

4. Setup metadata for the contract using the `archway metadata` command.

5. Check the history of the contract using the `archway history` command.

6. Query the contract state using the `archway query` command.

7. Execute transactions using the `archway tx` command, passing in the necessary parameters.

instantiate example: `archway instantiate --args '{"minter":"archway1qq65wjefu6nnqx0n6vvx5xzz3xmcuy75vauhq9", "name":"test", "symbol":"test"}'`

execute example: `archway tx --args '{"instantiate_stored_contract": {"code_id":633, "init_msg": "eyJtaW50ZXIiOiJhcmNod2F5MXFxNjV3amVmdTZubnF4MG42dnZ4NXh6ejN4bWN1eTc1dmF1aHE5IiwgIm5hbWUiOiJ0ZXN0IiwgInN5bWJvbCI6InRlc3QifQ==", "admin": "archway1qq65wjefu6nnqx0n6vvx5xzz3xmcuy75vauhq9", "label":"test"}}'`

## Instantiate Method

For more information on the `Instantiate` method, please refer to the following [documentation](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/enum.WasmMsg.html).

## TODO

In the future, a query method will be added to this contract to collect all executions in the smart contract state and receive all instantiated smart contracts within the Instantiator contract.

Thank you for using the Instantiator contract. If you have any questions or feedback, please feel free to contact us.
