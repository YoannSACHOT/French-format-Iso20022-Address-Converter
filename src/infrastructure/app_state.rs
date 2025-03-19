use crate::application::command::address_command_service::AddressCommandService;
use crate::application::query::address_query_service::AddressQueryService;
use std::sync::Mutex;

pub struct AppState {
    pub command_service: Mutex<AddressCommandService>,
    pub query_service: Mutex<AddressQueryService>,
}

impl AppState {
    pub fn new(command_service: AddressCommandService, query_service: AddressQueryService) -> Self {
        Self {
            command_service: Mutex::new(command_service),
            query_service: Mutex::new(query_service),
        }
    }
}