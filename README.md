```
As I found at the moment - it is impossible to directly instantiate another contract from within a contract on the CosmWasm platform.

In the Cosmos SDK and CosmWasm, the process of instantiating a new contract must be done off-chain or through an external transaction. The reason behind this limitation is that smart contracts in the Cosmos SDK are designed to be isolated from each other to enhance security and maintainability.

As an alternative, I can create a separate transaction that instantiates the new contract and submit that transaction from a client or another off-chain application. 

So seems I have to store users contracts with web2 features like database for query later all users instantiations and displaying them in ui... 
```

```
seems it is possible 
https://docs.rs/cosmwasm-std/latest/cosmwasm_std/enum.WasmMsg.html
Instantiate
Fields
admin: Option<String>
code_id: u64
msg: Binary
msg is the JSON-encoded InstantiateMsg struct (as raw Binary)

funds: Vec<Coin>
label: String
A human-readbale label for the contract

Instantiates a new contracts from previously uploaded Wasm code.

The contract address is non-predictable. But it is guaranteed that when emitting the same Instantiate message multiple times, multiple instances on different addresses will be generated. See also Instantiate2.

This is translated to a MsgInstantiateContract. sender is automatically filled with the current contract’s address.
```