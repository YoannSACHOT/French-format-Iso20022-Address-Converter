use crate::domain::models::{AddressKind, ISO20022Address};
use crate::domain::{models::FrenchAddress, repository::AddressRepository};

pub struct AddressService {
    repository: Box<dyn AddressRepository>,
}

impl AddressService {
    pub fn new(repository: Box<dyn AddressRepository>) -> Self {
        Self { repository }
    }

    pub fn add_address(&mut self, address: ISO20022Address) -> Result<(), String> {
        self.repository.save(address)
    }

    pub fn get_address(&self, address_id: &str) -> Option<ISO20022Address> {
        self.repository.find_by_id(address_id)
    }

    pub fn delete_address(&mut self, address_id: &str) -> Result<(), String> {
        self.repository.delete(address_id)
    }

    pub fn update_address(&mut self, address: ISO20022Address) -> Result<(), String> {
        self.repository.update(address)
    }

    pub fn get_all_addresses(&self) -> Vec<ISO20022Address> {
        self.repository.find_all()
    }

    pub fn convert_to_iso(&self, address: &FrenchAddress, kind: AddressKind) -> ISO20022Address {
        crate::domain::usecases::convert_to_iso(&address, kind)
    }

    pub fn convert_to_french(&self, address: &ISO20022Address) -> FrenchAddress {
        crate::domain::usecases::convert_to_french(&address)
    }
}
