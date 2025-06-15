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
    fn test_get_health_alerts() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create an animal
        let animal_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        
        // Update health status to trigger an alert
        contract.update_health_status(animal_id, HealthStatus::Sick);
        
        // Retrieve health alerts
        let alerts = contract.get_health_alerts();
        
        // Verify the alert
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

    #[test]
    fn test_get_animal() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create an animal
        let animal_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        
        // Retrieve the animal
        let animal = contract.get_animal(animal_id).unwrap();
        
        // Verify animal details
        assert_eq!(animal.age, 2);
        assert_eq!(animal.breed, "Angus");
        assert_eq!(animal.height, 1.5);
    }

    #[test]
    fn test_get_all_animals() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create multiple animals
        contract.create_animal(2, "Angus".to_string(), 1.5);
        contract.create_animal(3, "Holstein".to_string(), 1.6);
        
        // Retrieve all animals
        let animals = contract.livestock_manager.animals.iter().collect::<Vec<_>>();
        
        // Verify the number of animals
        assert_eq!(animals.len(), 2);
    }

    #[test]
    fn test_get_pedigree() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create parent animals
        let parent1_id = contract.create_animal(3, "Angus".to_string(), 1.6);
        let parent2_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        
        // Breed animals to create offspring
        let offspring_id = contract.breed_animals(parent1_id, parent2_id, "Angus".to_string()).unwrap();
        
        // Retrieve the pedigree of the offspring
        let pedigree = contract.livestock_manager.get_pedigree(offspring_id).unwrap();
        
        // Verify pedigree details
        assert_eq!(pedigree.parent1_id, parent1_id);
        assert_eq!(pedigree.parent2_id, parent2_id);
    }

    #[test]
    fn test_get_average_height() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create multiple animals with different heights
        contract.create_animal(2, "Angus".to_string(), 1.5);
        contract.create_animal(3, "Holstein".to_string(), 1.6);
        contract.create_animal(2, "Angus".to_string(), 1.4);
        
        // Calculate average height
        let average_height = contract.livestock_manager.get_average_height();
        
        // Verify average height
        assert_eq!(average_height, 1.5);
    }

    #[test]
    fn test_get_average_age() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create multiple animals with different ages
        contract.create_animal(2, "Angus".to_string(), 1.5);
        contract.create_animal(3, "Holstein".to_string(), 1.6);
        contract.create_animal(4, "Angus".to_string(), 1.4);
        
        // Calculate average age
        let average_age = contract.get_average_age();
        
        // Verify average age
        assert_eq!(average_age, 3.0);
    }

    #[test]
    fn test_get_sick_animals() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create animals with different health statuses
        let healthy_animal_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        let sick_animal_id = contract.create_animal(3, "Holstein".to_string(), 1.6);
        
        // Update one animal to sick status
        contract.update_health_status(sick_animal_id, HealthStatus::Sick);
        contract.update_health_status(healthy_animal_id, HealthStatus::Healthy);
        
        // Retrieve sick animals
        let sick_animals = contract.get_sick_animals();
        
        // Verify the sick animals
        assert_eq!(sick_animals.len(), 1);
        assert_eq!(sick_animals[0].id, sick_animal_id);
    }

    #[test]
    fn test_get_healthy_animals() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create animals with different health statuses
        let healthy_animal_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        let sick_animal_id = contract.create_animal(3, "Holstein".to_string(), 1.6);
        
        // Update one animal to sick status
        contract.update_health_status(sick_animal_id, HealthStatus::Sick);
        contract.update_health_status(healthy_animal_id, HealthStatus::Healthy);
        
        // Retrieve healthy animals
        let healthy_animals = contract.get_healthy_animals();
        
        // Verify the healthy animals
        assert_eq!(healthy_animals.len(), 1);
        assert_eq!(healthy_animals[0].id, healthy_animal_id);
    }

    #[test]
    fn test_get_critical_animals() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create animals with different health statuses
        let healthy_animal_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        let critical_animal_id = contract.create_animal(3, "Holstein".to_string(), 1.6);
        
        // Update one animal to critical status
        contract.update_health_status(critical_animal_id, HealthStatus::Critical);
        contract.update_health_status(healthy_animal_id, HealthStatus::Healthy);
        
        // Retrieve critical animals
        let critical_animals = contract.livestock_manager.get_critical_animals();
        
        // Verify the critical animals
        assert_eq!(critical_animals.len(), 1);
        assert_eq!(critical_animals[0].id, critical_animal_id);
    }

    #[test]
    fn test_get_recovering_animals() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create animals with different health statuses
        let healthy_animal_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        let recovering_animal_id = contract.create_animal(3, "Holstein".to_string(), 1.6);
        
        // Update one animal to recovering status
        contract.update_health_status(recovering_animal_id, HealthStatus::Recovering);
        contract.update_health_status(healthy_animal_id, HealthStatus::Healthy);
        
        // Retrieve recovering animals
        let recovering_animals = contract.livestock_manager.get_recovering_animals();
        
        // Verify the recovering animals
        assert_eq!(recovering_animals.len(), 1);
        assert_eq!(recovering_animals[0].id, recovering_animal_id);
    }

    #[test]
    fn test_get_animal_per_breed() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());

        // Create animals of different breeds
        let angus_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        let holstein_id = contract.create_animal(3, "Holstein".to_string(), 1.6);

        // Retrieve animals by breed
        let angus_animals = contract.get_animal_per_breed("Angus".to_string());
        let holstein_animals = contract.get_animal_per_breed("Holstein".to_string());

        // Verify the retrieved animals
        assert_eq!(angus_animals.len(), 1);
        assert_eq!(angus_animals[0].id, angus_id);

        assert_eq!(holstein_animals.len(), 1);
        assert_eq!(holstein_animals[0].id, holstein_id);
    }

    #[test]
    fn test_delete_animal() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SmartAgriContract::new("owner.near".parse().unwrap());
        
        // Create an animal
        let animal_id = contract.create_animal(2, "Angus".to_string(), 1.5);
        
        // Delete the animal
        assert!(contract.livestock_manager.delete_animal(animal_id));
        
        // Verify the animal is deleted
        assert!(contract.get_animal(animal_id).is_none());
    }
}
