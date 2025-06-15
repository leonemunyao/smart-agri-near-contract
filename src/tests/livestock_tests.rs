#[cfg(test)]
mod tests {
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};
    use crate::{SmartAgriContract, livestock::HealthStatus};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder.current_account_id("contract.near".parse().unwrap());
        builder.block_timestamp(1000000);
        builder
    }

    #[test]
    fn test_create_animal() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Test creating an animal
        let animal_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        assert_eq!(animal_id, 0); // First animal should have ID 0

        // Verify animal details
        let animal = contract.get_animal(animal_id).unwrap();
        assert_eq!(animal.age, 2);
        assert_eq!(animal.breed, "Angus");
        assert_eq!(animal.height, 1.5);
        assert_eq!(animal.healthstatus, HealthStatus::Healthy);
    }

    #[test]
    fn test_breeding() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract     = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create parent animals
        let parent1_id = contract.create_animal(3, "Angus".to_string(), 1.6);
        let parent2_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        
        // Test breeding
        let offspring_id = contract.breed_animals(parent1_id, parent2_id, "Angus".to_string());
        assert!(offspring_id.is_some());
        
        // Verify offspring details
        let offspring = contract.get_animal(offspring_id.unwrap()).unwrap();
        assert_eq!(offspring.age, 0);
        assert_eq!(offspring.breed, "Angus");
        assert!(offspring.parent_ids.is_some());
        let parents = offspring.parent_ids.unwrap();
        assert_eq!(parents.parent1_id, parent1_id);
        assert_eq!(parents.parent2_id, parent2_id);
    }

    #[test]
    fn test_health_management() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create an animal
        let animal_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        
        // Test health status update
        assert!(contract.update_health_status(animal_id, HealthStatus::Sick));
        
        // Verify health status
        let animal = contract.get_animal(animal_id).unwrap();
        assert_eq!(animal.healthstatus, HealthStatus::Sick);
        
        // Check health alerts
        let alerts = contract.get_health_alerts();
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].animal_id, animal_id);
        assert_eq!(alerts[0].status, HealthStatus::Sick);
    }

    #[test]
    fn test_medication() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create an animal
        let animal_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        
        // Add medication
        assert!(contract.add_medication(
            animal_id,
            "Antibiotics".to_string(),
            "10mg daily".to_string()
        ));
        
        // Verify medication records
        let animal = contract.get_animal(animal_id).unwrap();
        assert_eq!(animal.medical_records.len(), 1);
        assert_eq!(animal.medical_records[0].name, "Antibiotics");
        assert_eq!(animal.medical_records[0].dosage, "10mg daily");
    }

    #[test]
    fn test_statistics() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create multiple animals
        contract.create_animal(2, "Angus".to_string(), 1.5);
        contract.create_animal(3, "Holstein".to_string(), 1.6);
        contract.create_animal(2, "Angus".to_string(), 1.4);
        
        // Update some health statuses
        contract.update_health_status(1, HealthStatus::Sick);
        
        // Get statistics
        let stats = contract.get_livestock_statistics();
        
        // Verify statistics
        assert_eq!(stats.get("total_animals").unwrap(), &3);
        assert_eq!(stats.get("breed_angus").unwrap(), &2);
        assert_eq!(stats.get("breed_holstein").unwrap(), &1);
        assert_eq!(stats.get("health_sick").unwrap(), &1);
        assert_eq!(stats.get("health_healthy").unwrap(), &2);
    }
}
