use postgres::{Client, Error, NoTls};
use std::cell::RefCell;
use crate::domain::models::ISO20022Address;
use crate::domain::repository::AddressRepository;

pub struct PostgresAddressRepository {
    client: RefCell<Client>,
}

impl PostgresAddressRepository {
    pub fn new(connection_str: &str) -> Result<Self, Error> {
        let client = Client::connect(connection_str, NoTls)?;
        //TODO: create table here
        Ok(Self {
            client: RefCell::new(client),
        })
    }
}

impl AddressRepository for PostgresAddressRepository {
    fn save(&mut self, address: ISO20022Address) -> Result<(), String> {
        todo!()
    }

    fn update(&mut self, address: ISO20022Address) -> Result<(), String> {
        todo!()
    }

    fn delete(&mut self, address_id: &str) -> Result<(), String> {
        todo!()
    }

    fn find_by_id(&self, address_id: &str) -> Option<ISO20022Address> {
        todo!()
    }

    fn find_all(&self) -> Vec<ISO20022Address> {
        todo!()
    }
}
