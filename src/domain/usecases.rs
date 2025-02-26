use crate::domain::models::{FrenchAddress, ISO20022Address};

pub fn convert_to_iso(address: &FrenchAddress) -> ISO20022Address {
    ISO20022Address {
        ..Default::default()
    }
}
