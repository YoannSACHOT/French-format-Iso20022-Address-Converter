use fraddris020022::domain::models::FrenchAddress;

#[test]
pub fn convert_to_iso() {
    let address = FrenchAddress {
        line1: Option::from("".to_string()),
        line2: Option::from("".to_string()),
        line3: Option::from("".to_string()),
        line4: Option::from("".to_string()),
        line5: Option::from("".to_string()),
        line6: Option::from("".to_string()),
        line7: Option::from("".to_string()),
    };

    let iso_address = fraddris020022::domain::usecases::convert_to_iso(&address);
    assert_eq!(iso_address.department, address.line1);
}
