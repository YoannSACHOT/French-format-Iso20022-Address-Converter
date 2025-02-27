use crate::domain::models::ISO20022Address;
use crate::domain::usecases::convert_to_french;
use crate::domain::{
    models::FrenchAddress,
    repository::AddressRepository,
    usecases::{AddressKind, convert_to_iso},
};

pub struct AddressService<R: AddressRepository> {
    repository: R,
}

impl<R: AddressRepository> AddressService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn add_address(&mut self, address: ISO20022Address) -> Result<(), String> {
        self.repository.save(address)
    }

    pub fn convert_to_iso(&self, address: &FrenchAddress, kind: AddressKind) -> ISO20022Address {
        convert_to_iso(&address, kind)
    }

    pub fn convert_to_french(&self, address: &ISO20022Address) -> FrenchAddress {
        convert_to_french(&address)
    }

    pub fn get_all_addresses(&self) -> Vec<ISO20022Address> {
        self.repository.find_all()
    }

    pub fn get_address(&self, address_id: &str) -> Option<ISO20022Address> {
        self.repository.find_by_id(address_id)
    }

    pub fn update_address(&mut self, address: ISO20022Address) -> Result<(), String> {
        self.repository.update(address)
    }

    pub fn delete_address(&mut self, address_id: &str) -> Result<(), String> {
        self.repository.delete(address_id)
    }
}
