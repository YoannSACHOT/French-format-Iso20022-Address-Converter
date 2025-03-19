use crate::domain::models::ISO20022Address;
use crate::domain::repository::ReadAddressRepository;

pub struct AddressQueryService {
    repository: Box<dyn ReadAddressRepository>,
}

impl AddressQueryService {
    pub fn new(repository: Box<dyn ReadAddressRepository>) -> Self {
        Self { repository }
    }

    pub fn get_address(&self, address_id: &str) -> Option<ISO20022Address> {
        self.repository.find_by_id(address_id)
    }

    pub fn get_all_addresses(&self) -> Vec<ISO20022Address> {
        self.repository.find_all()
    }
}
