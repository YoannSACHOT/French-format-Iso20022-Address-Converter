use crate::domain::models::ISO20022Address;
use crate::domain::repository::{AddressRepository, ReadAddressRepository};
use serde_json;
use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
pub struct FileBasedAddressRepository {
    file_path: String,
}

impl FileBasedAddressRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    fn load_data(&self) -> HashMap<String, ISO20022Address> {
        let data = fs::read_to_string(&self.file_path).unwrap_or("{}".to_string());
        serde_json::from_str(&data).unwrap_or_else(|_| HashMap::new())
    }

    fn save_data(&self, data: &HashMap<String, ISO20022Address>) {
        let json = serde_json::to_string_pretty(data).unwrap();
        fs::write(&self.file_path, json).expect("File write failed");
    }
}

impl AddressRepository for FileBasedAddressRepository {
    fn save(&mut self, address: ISO20022Address) -> Result<(), String> {
        let mut data = self.load_data();
        data.insert(address.id.clone(), address);
        self.save_data(&data);
        Ok(())
    }

    fn update(&mut self, address: ISO20022Address) -> Result<(), String> {
        let mut data = self.load_data();
        if data.contains_key(&address.id) {
            data.insert(address.id.clone(), address);
            self.save_data(&data);
            Ok(())
        } else {
            Err("Address not found".to_string())
        }
    }

    fn delete(&mut self, address_id: &str) -> Result<(), String> {
        let mut data = self.load_data();
        if data.remove(address_id).is_some() {
            self.save_data(&data);
            Ok(())
        } else {
            Err("Address not found".to_string())
        }
    }
}

impl ReadAddressRepository for FileBasedAddressRepository {
    fn find_by_id(&self, address_id: &str) -> Option<ISO20022Address> {
        self.load_data().get(address_id).cloned()
    }

    fn find_all(&self) -> Vec<ISO20022Address> {
        self.load_data().values().cloned().collect()
    }
}
