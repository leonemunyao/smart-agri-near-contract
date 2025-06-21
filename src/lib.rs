use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault, env};
use std::collections::HashMap;


#[cfg(test)]
mod tests {
    pub mod livestock_tests;
}

mod livestock;



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SmartAgriContract {
    pub owner_id: AccountId,
    pub livestock_manager: livestock::LivestockManager,
}

#[near_bindgen]
impl SmartAgriContract {
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Contract is already initialized");
        Self {
            owner_id,
            livestock_manager: livestock::LivestockManager::new()
        }
    }

    // Livestock management functions
    pub fn create_animal(&mut self, age: u8, breed: String, height: u32) -> u64 {
        self.livestock_manager.create_animal(age, breed, height)
    }

    pub fn breed_animals(&mut self, parent1_id: u64, parent2_id: u64, breed: String) -> Option<u64> {
        self.livestock_manager.breed_animals(parent1_id, parent2_id, breed)
    }

    pub fn update_health_status(&mut self, id: String, new_status: livestock::HealthStatus) -> bool {
        self.livestock_manager.update_health_status(id.to_string(), new_status)
    }

    pub fn add_medication(&mut self, animal_id: String, name: String, dosage: String) -> bool {
        self.livestock_manager.add_medication(animal_id.to_string(), name, dosage)
    }

    pub fn get_animal(&self, id: String) -> Option<livestock::Livestock> {
        self.livestock_manager.get_animal(id.to_string())
    }

    pub fn get_health_alerts(&self) -> Vec<livestock::HealthAlert> {
        self.livestock_manager.get_health_alerts()
    }

    pub fn get_livestock_statistics(&self) -> HashMap<String, u64> {
        self.livestock_manager.get_statistics()
    }
    pub fn delete_animal(&mut self, id: String) -> bool {
        self.livestock_manager.delete_animal(id)
    }

    pub fn get_all_animals(&self) -> Vec<livestock::Livestock> {
        self.livestock_manager.get_all_animals()
    }

    pub fn get_pedigree(&self, animal_id: String) -> Option<livestock::ParentIds> {
        self.livestock_manager.get_pedigree(animal_id)
    }

    pub fn get_average_age(&self) -> u32 {
        self.livestock_manager.get_average_age()
    }

    pub fn get_average_height(&self) -> u32 {
        self.livestock_manager.get_average_height()
    }

    pub fn get_sick_animals(&self) -> Vec<livestock::Livestock> {
        self.livestock_manager.get_sick_animals()
    }

    pub fn get_healthy_animals(&self) -> Vec<livestock::Livestock> {
        self.livestock_manager.get_healthy_animals()
    }

    pub fn get_critical_animals(&self) -> Vec<livestock::Livestock> {
        self.livestock_manager.get_critical_animals()
    }

    pub fn get_recovering_animals(&self) -> Vec<livestock::Livestock> {
        self.livestock_manager.get_recovering_animals()
    }

    pub fn get_animal_per_breed(&self, breed: String) -> Vec<livestock::Livestock> {
        self.livestock_manager.get_animal_per_breed(&breed)
    }

    // Owner Management
    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

}

