## Testing the Near Smart Contract.

The first step is to create a Near testnet account and make it accessible via `near cli`

8 First make sure near cli is installed:

`npm install -g near-cli`

* Build the contract:

`cargo build --target wasm32-unknown-unknown --release`


## Deployment Steps

1. Create a sub-account for the contract:
```bash
near create-account smart-agri.YOUR_ACCOUNT.testnet --masterAccount YOUR_ACCOUNT.testnet --initialBalance 10
```

2. Deploy the contract:
```bash
near deploy smart-agri.YOUR_ACCOUNT.testnet \
    target/wasm32-unknown-unknown/release/smart_agri_near_contract.wasm \
    --initFunction 'new' \
    --initArgs '{"owner_id": "YOUR_ACCOUNT.testnet"}'
```

* You can then interact with your contract using NEAR CLI. Here are some example commands:

##### Create an animal
`near call YOUR_TESTNET_ACCOUNT.testnet create_animal '{"age": 2, "breed": "Angus", "height": 1.5}' --accountId YOUR_TESTNET_ACCOUNT.testnet`

##### Get animal details
`near view YOUR_TESTNET_ACCOUNT.testnet get_animal '{"id": 1}'`

##### Update health status
`near call YOUR_TESTNET_ACCOUNT.testnet update_health_status '{"id": 1 "new_status": "Sick"}' --accountId YOUR_TESTNET_ACCOUNT.testnet`

##### Get statistics
`near view YOUR_TESTNET_ACCOUNT.testnet get_livestock_statistics '{}'`# smart-agri-near-contract
