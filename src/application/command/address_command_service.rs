use crate::domain::models::ISO20022Address;
use crate::domain::repository::AddressRepository;

pub struct AddressCommandService {
    repository: Box<dyn AddressRepository>,
}

impl AddressCommandService {
    pub fn new(repository: Box<dyn AddressRepository>) -> Self {
        Self { repository }
    }

    pub fn add_address(&mut self, address: ISO20022Address) -> Result<(), String> {
        self.repository.save(address)
    }

    pub fn delete_address(&mut self, address_id: &str) -> Result<(), String> {
        self.repository.delete(address_id)
    }

    pub fn update_address(&mut self, address: ISO20022Address) -> Result<(), String> {
        self.repository.update(address)
    }
}
