use std::sync::Mutex;
use crate::application::address_service::AddressService;

pub struct AppState {
    pub service: Mutex<AddressService>,
}