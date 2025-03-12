use crate::domain::models::ISO20022Address;

pub trait AddressRepository: Send + Sync {
    fn save(&mut self, address: ISO20022Address) -> Result<(), String>;
    fn update(&mut self, address: ISO20022Address) -> Result<(), String>;
    fn delete(&mut self, address_id: &str) -> Result<(), String>;
    fn find_by_id(&self, address_id: &str) -> Option<ISO20022Address>;
    fn find_all(&self) -> Vec<ISO20022Address>;
}
