use fraddris020022::domain::models::ISO20022Address;
use fraddris020022::domain::repository::AddressRepository;
use std::collections::HashMap;

pub struct InMemoryAddressRepository {
    addresses: HashMap<String, ISO20022Address>,
}

impl InMemoryAddressRepository {
    pub fn new() -> Self {
        Self {
            addresses: HashMap::new(),
        }
    }
}

impl AddressRepository for InMemoryAddressRepository {
    fn save(&mut self, address: ISO20022Address) -> Result<(), String> {
        self.addresses.insert(address.id.clone(), address);
        Ok(())
    }

    fn update(&mut self, address: ISO20022Address) -> Result<(), String> {
        if self.addresses.contains_key(&address.id) {
            self.addresses.insert(address.id.clone(), address);
            Ok(())
        } else {
            Err("Adresse non trouvée".to_string())
        }
    }

    fn delete(&mut self, address_id: &str) -> Result<(), String> {
        if self.addresses.remove(address_id).is_some() {
            Ok(())
        } else {
            Err("Adresse non trouvée".to_string())
        }
    }

    fn find_by_id(&self, address_id: &str) -> Option<ISO20022Address> {
        self.addresses.get(address_id).cloned()
    }

    fn find_all(&self) -> Vec<ISO20022Address> {
        self.addresses.values().cloned().collect()
    }
}
