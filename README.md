## Livestock Management System

A blockchain-based livestock management system built on the NEAR Protocol that enables efficient tracking and management of livestock data.

### Overview

This smart contract provides comprehensive functionality for managing livestock including health tracking, breeding records, and statistical analysis.

#### Features
* Animal registration and management
* Health status monitoring
* Breeding records
* Medication tracking
* Statistical analysis
* Pedigree tracking

#### Contract Functions

##### Animal Management

* create_animal(age: u8, breed: String, height: f32) -> u64

Creates a new animal record
Returns the unique ID of the created animal

* get_animal(id: u64) -> Option<Livestock>

Retrieves details of a specific animal

* get_all_animals() -> Vec<Livestock>

Returns a list of all registered animals

* delete_animal(id: u64) -> bool

Removes an animal from the system
Returns true if successful

##### Health Management

* update_health_status(id: u64, new_status: HealthStatus) -> bool

Updates an animal's health status
Generates health alerts for non-healthy statuses

* add_medication(animal_id: u64, name: String, dosage: String) -> bool

Records medication given to an animal

* get_health_alerts() -> Vec<HealthAlert>

Retrieves all health alerts in the system

##### Breeding Management
* breed_animals(parent1_id: u64, parent2_id: u64, breed: String) -> Option<u64>

Records breeding between two animals
Returns the ID of the offspring

* get_pedigree(animal_id: u64) -> Option<ParentIds>

Retrieves the parent IDs for an animal

##### Analytics
* get_livestock_statistics() -> HashMap<String, u64>

Returns comprehensive statistics about the livestock

* get_average_age() -> f32

Calculates average age of all animals

* get_average_height() -> f32

Calculates average height of all animals

##### Health Status Queries
* get_sick_animals() -> Vec<Livestock>
* get_healthy_animals() -> Vec<Livestock>
* get_critical_animals() -> Vec<Livestock>
* get_recovering_animals() -> Vec<Livestock>
* get_animal_per_breed(breed: String) -> Vec<Livestock>



## Testing the Near Smart Contract.

The first step is to create a Near testnet account and make it accessible via `near cli`

8 First make sure near cli is installed:

`npm install -g near-cli`

* Build the contract:

`cargo build --target wasm32-unknown-unknown --release`


## Deployment Steps

1. Create a sub-account for the contract:
```bash
near create-account livestock.YOUR_ACCOUNT.testnet --masterAccount YOUR_ACCOUNT.testnet --initialBalance 10
```

2. Deploy the contract:
```bash
near deploy livestock.YOUR_ACCOUNT.testnet \
    target/wasm32-unknown-unknown/release/smart_agri_near_contract.wasm
```

* You can then interact with your contract using NEAR CLI. Here are some example commands:

##### Create an animal
`near call livestock.YOUR_ACCOUNT.testnet create_animal '{"age": 2, "breed": "Angus", "height": 1.5}' --accountId YOUR_TESTNET_ACCOUNT.testnet`

##### Get animal details
`near view livestock.YOUR_ACCOUNT.testnet get_animal '{"id": 1}'`

##### Update health status
`near call livestock.YOUR_ACCOUNT.testnet update_health_status '{"id": 1 "new_status": "Sick"}' --accountId YOUR_TESTNET_ACCOUNT.testnet`

##### Get statistics
`near view livestock.YOUR_ACCOUNT.testnet get_livestock_statistics '{}'`# smart-agri-near-contract
