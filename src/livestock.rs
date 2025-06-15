use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, BorshStorageKey};
use near_sdk::collections::UnorderedMap;
use std::collections::HashMap;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Animals,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Livestock {
    pub id: u64,
    pub breed: String,
    pub age: u8,
    pub height: f32,
    pub healthrecords: String,
    pub healthstatus: HealthStatus,
    pub medical_records: Vec<Medication>,
    pub parent_ids: Option<ParentIds>,
    pub created_at: u64,
    pub updated_at: Option<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Medication {
    pub id: u64,
    pub name: String,
    pub dosage: String,
    pub start_date: u64,
    pub end_date: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ParentIds {
    pub parent1_id: u64,
    pub parent2_id: u64,

}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct HealthAlert {
    pub animal_id: u64,
    pub status: HealthStatus,
    pub timestamp: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(crate = "near_sdk::serde")]
pub enum HealthStatus {
    Healthy,
    Sick,
    Critical,
    Recovering,
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct LivestockManager {
    pub animals: UnorderedMap<u64, Livestock>,
    pub health_alerts: Vec<HealthAlert>,
    pub next_id: u64,
}

impl LivestockManager {
    pub fn new() -> Self {
        Self {
            animals: UnorderedMap::new(StorageKey::Animals),
            health_alerts: Vec::new(),
            next_id: 0,
        }
    }
}

impl Default for LivestockManager {
    fn default() -> Self {
        Self::new()
    }
}


impl LivestockManager {
    pub fn create_animal(&mut self, age: u8, breed: String, height: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let animal = Livestock {
            id,
            age,
            breed,
            height,
            healthrecords: "Healthy".to_string(),
            healthstatus: HealthStatus::Healthy,
            medical_records: Vec::new(),
            parent_ids: None,
            created_at: env::block_timestamp(),
            updated_at: None,
        };

        self.animals.insert(&id, &animal);
        id
    }

    pub fn breed_animals(&mut self, parent1_id: u64, parent2_id: u64, breed: String) -> Option<u64> {
        if self.animals.get(&parent1_id).is_some() && self.animals.get(&parent2_id).is_some() {
            let offspring_id = self.create_animal(0, breed, 0.0);
            let mut offspring = self.animals.get(&offspring_id).unwrap();
            
            offspring.parent_ids = Some(ParentIds {
                parent1_id,
                parent2_id,
            });
            
            self.animals.insert(&offspring_id, &offspring);
            Some(offspring_id)
        } else {
            None
        }
    }

    pub fn update_health_status(&mut self, id: u64, new_status: HealthStatus) -> bool {
        if let Some(mut animal) = self.animals.get(&id) {
            animal.healthstatus = new_status.clone();
            animal.healthrecords = format!("{:?}", new_status);
            animal.updated_at = Some(env::block_timestamp());

            if new_status != HealthStatus::Healthy {
                self.health_alerts.push(HealthAlert {
                    animal_id: id,
                    status: new_status,
                    timestamp: env::block_timestamp(),
                });
            }

            self.animals.insert(&id, &animal);
            true
        } else {
            false
        }
    }

    pub fn add_medication(&mut self, animal_id: u64, name: String, dosage: String) -> bool {
        if let Some(mut animal) = self.animals.get(&animal_id) {
            let medication = Medication {
                id: animal.medical_records.len() as u64 + 1,
                name,
                dosage,
                start_date: env::block_timestamp(),
                end_date: env::block_timestamp() + 86400_000_000_000, // 1 day in nanoseconds
            };
            
            animal.medical_records.push(medication);
            self.animals.insert(&animal_id, &animal);
            true
        } else {
            false
        }
    }

    pub fn delete_animal(&mut self, id: u64) -> bool {
        if self.animals.remove(&id).is_some() {
            self.health_alerts.retain(|alert| alert.animal_id != id);
            true
        } else {
            false
        }
    }

    // Query methods
    pub fn get_animal(&self, id: u64) -> Option<Livestock> {
        self.animals.get(&id)
    }

    pub fn get_all_animals(&self) -> Vec<Livestock> {
        self.animals.values().collect()
    }

    pub fn get_health_alerts(&self) -> Vec<HealthAlert> {
        self.health_alerts.clone()
    }

    pub fn get_pedigree(&self, animal_id: u64) -> Option<ParentIds> {
        self.animals.get(&animal_id).and_then(|animal| animal.parent_ids)
    }

    pub fn get_average_age(&self) -> f32 {
        let total_age: u32 = self.animals.values().map(|animal| animal.age as u32).sum();
        let count = self.animals.len() as u32;
        if count == 0 {
            0.0
        } else {
            total_age as f32 / count as f32
        }
    }

    pub fn get_average_height(&self) -> f32 {
        let total_height: f32 = self.animals.values().map(|animal| animal.height).sum();
        let count = self.animals.len() as u32;
        if count == 0 {
            0.0
        } else {
            total_height / count as f32
        }
    }

    pub fn get_sick_animals(&self) -> Vec<Livestock> {
        self.animals.values()
            .filter(|animal| animal.healthstatus == HealthStatus::Sick)
            .collect()
    }

    pub fn get_healthy_animals(&self) -> Vec<Livestock> {
        self.animals.values()
            .filter(|animal| animal.healthstatus == HealthStatus::Healthy)
            .collect()
    }

    pub fn get_critical_animals(&self) -> Vec<Livestock> {
        self.animals.values()
            .filter(|animal| animal.healthstatus == HealthStatus::Critical)
            .collect()
    }

    pub fn get_recovering_animals(&self) -> Vec<Livestock> {
        self.animals.values()
            .filter(|animal| animal.healthstatus == HealthStatus::Recovering)
            .collect()
    }

    pub fn get_animal_per_breed(&self, breed: &str) -> Vec<Livestock> {
        self.animals.values()
            .filter(|animal| animal.breed.to_lowercase() == breed.to_lowercase())
            .collect()
    }

    pub fn get_statistics(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        let mut health_counts = HashMap::new();
        let mut breed_counts = HashMap::new();
        
        for animal in self.animals.values() {
            *health_counts.entry(format!("{:?}", animal.healthstatus)).or_insert(0) += 1;
            *breed_counts.entry(animal.breed.clone()).or_insert(0) += 1;
        }
        
        stats.insert("total_animals".to_string(), self.animals.len());
        
        // Add health status counts
        for (status, count) in health_counts {
            stats.insert(format!("health_{}", status.to_lowercase()), count);
        }
        
        // Add breed counts
        for (breed, count) in breed_counts {
            stats.insert(format!("breed_{}", breed.to_lowercase()), count);
        }
        
        stats
    }
}
