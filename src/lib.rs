// Contract entry points and initialization

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault, env};
use near_sdk::collections::UnorderedMap;
use std::collections::HashMap;
// use crate::livestock::{Livestock, HealthStatus};


#[cfg(test)]
mod tests {
    pub mod livestock_tests;
}

// mod crop;
mod livestock;
// mod defi;
// mod types;
// mod market;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SmartAgriContract {
    owner_id: String,
    livestock_manager: livestock::LivestockManager,
}

#[near_bindgen]
impl SmartAgriContract {
    #[init]
    pub fn new(owner_id: String) -> Self {
        assert!(!env::state_exists(), "Contract is already initialized");
        
        Self {
            owner_id: owner_id,
            livestock_manager: livestock::LivestockManager {
                animals: UnorderedMap::new(b"a"),
                health_alerts: Vec::new(),
                next_id: 0,
            },
        }
    }

    // Livestock management functions
    pub fn create_animal(&mut self, age: u8, breed: String, height: f32) -> u64 {
        self.livestock_manager.create_animal(age, breed, height)
    }

    pub fn breed_animals(&mut self, parent1_id: u64, parent2_id: u64, breed: String) -> Option<u64> {
        self.livestock_manager.breed_animals(parent1_id, parent2_id, breed)
    }

    pub fn update_health_status(&mut self, id: u64, new_status: livestock::HealthStatus) -> bool {
        self.livestock_manager.update_health_status(id, new_status)
    }

    pub fn add_medication(&mut self, animal_id: u64, name: String, dosage: String) -> bool {
        self.livestock_manager.add_medication(animal_id, name, dosage)
    }

    pub fn get_animal(&self, id: u64) -> Option<livestock::Livestock> {
        self.livestock_manager.get_animal(id)
    }

    pub fn get_health_alerts(&self) -> Vec<livestock::HealthAlert> {
        self.livestock_manager.get_health_alerts()
    }

    pub fn get_livestock_statistics(&self) -> HashMap<String, u64> {
        self.livestock_manager.get_statistics()
    }
}

