use fraddriso20022::domain::models::{FrenchAddress, ISO20022Address};
use fraddriso20022::domain::validation::{
    ValidationError, validate_french_address, validate_iso20022_address,
};

#[test]
fn test_french_address_validation_missing_line6() {
    let addr = FrenchAddress {
        id: "1".to_string(),
        line1: Some("Exemple".to_string()),
        line2: None,
        line3: None,
        line4: Some("10 rue de la Paix".to_string()),
        line5: None,
        line6: None,
        line7: Some("France".to_string()),
    };

    let res = validate_french_address(&addr);
    assert!(matches!(res, Err(ValidationError::EmptyField { field }) if field == "line6"));
}

#[test]
fn test_french_address_validation_invalid_postal_code() {
    let addr = FrenchAddress {
        id: "2".to_string(),
        line1: Some("Exemple".to_string()),
        line2: None,
        line3: None,
        line4: Some("10 rue de la Paix".to_string()),
        line5: None,
        line6: Some("ABC PARIS".to_string()),
        line7: Some("France".to_string()),
    };

    let res = validate_french_address(&addr);
    assert!(
        matches!(res, Err(ValidationError::InvalidPostalCode { value }) if value == "ABC PARIS")
    );
}

#[test]
fn test_iso20022_address_validation_invalid_country() {
    let mut iso = ISO20022Address::default();
    iso.id = "3".to_string();
    iso.street_name = Some("Main Street".to_string());
    iso.building_number = Some("10".to_string());
    iso.post_code = Some("75000".to_string());
    iso.town_name = Some("PARIS".to_string());
    iso.country = Some("FRANCE".to_string());

    let res = validate_iso20022_address(&iso);
    assert!(matches!(res, Err(ValidationError::InvalidCountryCode { value }) if value == "FRANCE"));
}
