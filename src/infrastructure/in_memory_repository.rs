use std::collections::HashMap;
use crate::domain::models::FrenchAddress;

#[derive(Default)]
pub struct InMemoryRepository {
    addresses: HashMap<String, FrenchAddress>,
}