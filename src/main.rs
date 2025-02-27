mod application;
mod domain;
mod infrastructure;

use crate::application::address_service::AddressService;
use crate::infrastructure::file_repository::FileBasedAddressRepository;

fn main() {
    let file_repo = FileBasedAddressRepository::new("addresses.json".to_string());
    let service = AddressService::new(file_repo);

    println!("{:?}", service.get_all_addresses());
}
