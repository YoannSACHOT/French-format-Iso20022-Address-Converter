use fraddris020022::application::address_service::AddressService;
use fraddris020022::domain::models::{FrenchAddress, ISO20022Address};
use fraddris020022::domain::usecases::AddressKind;

use fraddris020022::domain::repository::AddressRepository;
use fraddris020022::infrastructure::in_memory_repository::InMemoryAddressRepository;
use uuid::Uuid;

#[test]
fn particular_with_all_data() {
    let repo: Box<dyn AddressRepository> = Box::new(InMemoryAddressRepository::new());
    let mut service = AddressService::new(repo);

    let id = Uuid::new_v4().to_string();

    let address = FrenchAddress {
        id: id.clone(),
        line1: Some("Josh Homme".to_string()),
        line2: Some("Apt. 32".to_string()),
        line3: Some("Entrée 4".to_string()),
        line4: Some("10 rue de la Paix".to_string()),
        line5: Some("BP 52211".to_string()),
        line6: Some("88000 EPINAL".to_string()),
        line7: Some("France".to_string()),
    };

    let converted_address = service.convert_to_iso(&address, AddressKind::Particular);

    assert_eq!(converted_address.id, id);

    service
        .add_address(converted_address.clone())
        .expect("Failed to add address");

    let stored_address = service.get_address(&id);

    assert!(stored_address.is_some(), "Address not found in repository");
    let stored_address = stored_address.unwrap();

    assert_eq!(stored_address.id, id);
    assert_eq!(stored_address.department, None); // Pas défini pour un particulier
    assert_eq!(stored_address.sub_department, None); // Pas défini pour un particulier
    assert_eq!(stored_address.building_name, None);
    assert_eq!(stored_address.floor, Some("Entrée 4".to_string()));
    assert_eq!(stored_address.room, Some("Apt. 32".to_string()));
    assert_eq!(
        stored_address.street_name,
        Some("rue de la Paix".to_string())
    );
    assert_eq!(stored_address.building_number, Some("10".to_string()));
    assert_eq!(stored_address.post_box, Some("BP 52211".to_string()));
    assert_eq!(stored_address.town_location_name, None);
    assert_eq!(stored_address.post_code, Some("88000".to_string()));
    assert_eq!(stored_address.town_name, Some("EPINAL".to_string()));
    assert_eq!(stored_address.country, Some("FR".to_string())); // Vérification du mapping en FR
    assert_eq!(stored_address.district_name, None);
    assert_eq!(stored_address.country_sub_division, None);

    let address_update = ISO20022Address {
        id: id.clone(),
        recipient_name: Some("Josh Homme".to_string()),
        kind: AddressKind::Particular,
        department: None,
        sub_department: None,
        building_name: Some("Entrée 6".to_string()),
        floor: Some("3rd Floor".to_string()),
        room: Some("Apt. 32".to_string()),
        street_name: Some("rue 2 la Paix".to_string()),
        building_number: Some("11".to_string()),
        post_box: Some("BP 52222".to_string()),
        town_location_name: None,
        post_code: Some("75000".to_string()),
        town_name: Some("PARIS".to_string()),
        country: Some("FR".to_string()),
        district_name: None,
        country_sub_division: None,
    };

    service
        .update_address(address_update)
        .expect("Failed to update address");

    let stored_address = service.get_address(&id);
    assert!(stored_address.is_some(), "Address not found in repository");
    let stored_address = stored_address.unwrap();

    assert_eq!(stored_address.id, id);
    assert_eq!(stored_address.department, None); // Pas défini pour un particulier
    assert_eq!(stored_address.sub_department, None); // Pas défini pour un particulier
    assert_eq!(stored_address.building_name, Some("Entrée 6".to_string()));
    assert_eq!(stored_address.floor, Some("3rd Floor".to_string()));
    assert_eq!(stored_address.room, Some("Apt. 32".to_string()));
    assert_eq!(
        stored_address.street_name,
        Some("rue 2 la Paix".to_string())
    );
    assert_eq!(stored_address.building_number, Some("11".to_string()));
    assert_eq!(stored_address.post_box, Some("BP 52222".to_string()));
    assert_eq!(stored_address.town_location_name, None);
    assert_eq!(stored_address.post_code, Some("75000".to_string()));
    assert_eq!(stored_address.town_name, Some("PARIS".to_string()));
    assert_eq!(stored_address.country, Some("FR".to_string())); // Vérification du mapping en FR
    assert_eq!(stored_address.district_name, None);
    assert_eq!(stored_address.country_sub_division, None);

    let stored_address = service.get_address(&id);
    assert!(stored_address.is_some(), "Address not found in repository");

    service
        .delete_address(&id)
        .expect("Failed to delete address");

    let stored_address = service.get_address(&id);
    assert!(
        stored_address.is_none(),
        "Address was not deleted from repository !"
    );
}

#[test]
fn company_with_all_data() {
    let repo: Box<dyn AddressRepository> = Box::new(InMemoryAddressRepository::new());
    let mut service = AddressService::new(repo);

    let id = Uuid::new_v4().to_string();
    let address = FrenchAddress {
        id: id.clone(),
        line1: Some("DURAND SA".to_string()),
        line2: Some("Service achat".to_string()),
        line3: Some("Zone industrielle de la Ballastière Ouest".to_string()),
        line4: Some("22BIS RUE DES FLEURS".to_string()),
        line5: Some("BP 40122".to_string()),
        line6: Some("33506 LIBOURNE CEDEX".to_string()),
        line7: Some("France".to_string()),
    };

    let converted_address = service.convert_to_iso(&address, AddressKind::Company);

    assert_eq!(converted_address.id, id);

    service
        .add_address(converted_address.clone())
        .expect("Failed to add address");

    let stored_address = service.get_address(&id);

    assert!(stored_address.is_some(), "Address not found in repository");
    let stored_address = stored_address.unwrap();

    assert_eq!(stored_address.id, id);
    assert_eq!(stored_address.department, Some("Service achat".to_string()));
    assert_eq!(
        stored_address.floor,
        Some("Zone industrielle de la Ballastière Ouest".to_string())
    );
    assert_eq!(
        stored_address.street_name,
        Some("RUE DES FLEURS".to_string())
    );
    assert_eq!(stored_address.building_number, Some("22BIS".to_string()));
    assert_eq!(stored_address.post_box, Some("BP 40122".to_string()));
    assert_eq!(stored_address.post_code, Some("33506".to_string()));
    assert_eq!(stored_address.town_name, Some("LIBOURNE CEDEX".to_string()));
    assert_eq!(stored_address.country, Some("FR".to_string()));

    let address_update = ISO20022Address {
        id: id.clone(),
        recipient_name: Some("DURAND SA".to_string()),
        kind: AddressKind::Company,
        department: Some("COMPTABILITE".to_string()),
        sub_department: Some("BILANS".to_string()),
        building_name: Some("Entrée 6".to_string()),
        floor: Some("3rd Floor".to_string()),
        room: Some("Apt. 32".to_string()),
        street_name: Some("rue 2 la Paix".to_string()),
        building_number: Some("11".to_string()),
        post_box: Some("BP 52222".to_string()),
        town_location_name: None,
        post_code: Some("75000".to_string()),
        town_name: Some("PARIS".to_string()),
        country: Some("FR".to_string()),
        district_name: None,
        country_sub_division: None,
    };

    service
        .update_address(address_update)
        .expect("Failed to update address");

    let stored_address = service.get_address(&id);
    assert!(stored_address.is_some(), "Address not found in repository");
    let stored_address = stored_address.unwrap();

    assert_eq!(stored_address.id, id);
    assert_eq!(stored_address.department, Some("COMPTABILITE".to_string()));
    assert_eq!(stored_address.sub_department, Some("BILANS".to_string()));
    assert_eq!(stored_address.building_name, Some("Entrée 6".to_string()));
    assert_eq!(stored_address.floor, Some("3rd Floor".to_string()));
    assert_eq!(stored_address.room, Some("Apt. 32".to_string()));
    assert_eq!(
        stored_address.street_name,
        Some("rue 2 la Paix".to_string())
    );
    assert_eq!(stored_address.building_number, Some("11".to_string()));
    assert_eq!(stored_address.post_box, Some("BP 52222".to_string()));
    assert_eq!(stored_address.town_location_name, None);
    assert_eq!(stored_address.post_code, Some("75000".to_string()));
    assert_eq!(stored_address.town_name, Some("PARIS".to_string()));
    assert_eq!(stored_address.country, Some("FR".to_string())); // Vérification du mapping en FR
    assert_eq!(stored_address.district_name, None);
    assert_eq!(stored_address.country_sub_division, None);

    let stored_address = service.get_address(&id);
    assert!(stored_address.is_some(), "Address not found in repository");

    service
        .delete_address(&id)
        .expect("Failed to delete address");

    let stored_address = service.get_address(&id);
    assert!(
        stored_address.is_none(),
        "Address was not deleted from repository !"
    );
}

#[test]
fn private_individual_with_apartment() {
    let repo: Box<dyn AddressRepository> = Box::new(InMemoryAddressRepository::new());
    let service = AddressService::new(repo);

    let id = Uuid::new_v4().to_string();
    let address = FrenchAddress {
        id: id.clone(),
        line1: Some("Jean DURAND".to_string()),
        line2: Some("Apt. 12B".to_string()),
        line3: Some("3rd Floor".to_string()),
        line4: Some("10 RUE DES LILAS".to_string()),
        line5: None,
        line6: Some("75010 PARIS".to_string()),
        line7: Some("France".to_string()),
    };

    let converted_address = service.convert_to_iso(&address, AddressKind::Particular);

    assert_eq!(converted_address.id, id);
    assert_eq!(converted_address.room, Some("Apt. 12B".to_string()));
    assert_eq!(converted_address.floor, Some("3rd Floor".to_string()));
    assert_eq!(
        converted_address.street_name,
        Some("RUE DES LILAS".to_string())
    );
    assert_eq!(converted_address.building_number, Some("10".to_string()));
    assert_eq!(converted_address.post_code, Some("75010".to_string()));
    assert_eq!(converted_address.town_name, Some("PARIS".to_string()));
    assert_eq!(converted_address.country, Some("FR".to_string()));
}

#[test]
fn company_without_department() {
    let repo: Box<dyn AddressRepository> = Box::new(InMemoryAddressRepository::new());
    let service = AddressService::new(repo);

    let id = Uuid::new_v4().to_string();
    let address = FrenchAddress {
        id: id.clone(),
        line1: Some("LECLERC HYPERMARCHÉ".to_string()),
        line2: None,
        line3: None,
        line4: Some("1 AVENUE DE L'EUROPE".to_string()),
        line5: None,
        line6: Some("64000 PAU".to_string()),
        line7: Some("France".to_string()),
    };

    let converted_address = service.convert_to_iso(&address, AddressKind::Company);

    assert_eq!(converted_address.id, id);
    assert_eq!(
        converted_address.street_name,
        Some("AVENUE DE L'EUROPE".to_string())
    );
    assert_eq!(converted_address.building_number, Some("1".to_string()));
    assert_eq!(converted_address.post_code, Some("64000".to_string()));
    assert_eq!(converted_address.town_name, Some("PAU".to_string()));
    assert_eq!(converted_address.country, Some("FR".to_string()));
}

#[test]
fn private_individual_with_po_box() {
    let repo: Box<dyn AddressRepository> = Box::new(InMemoryAddressRepository::new());
    let service = AddressService::new(repo);

    let id = Uuid::new_v4().to_string();
    let address = FrenchAddress {
        id: id.clone(),
        line1: Some("Claire MARTIN".to_string()),
        line2: None,
        line3: None,
        line4: None,
        line5: Some("BP 1234".to_string()),
        line6: Some("31000 TOULOUSE".to_string()),
        line7: Some("France".to_string()),
    };

    let converted_address = service.convert_to_iso(&address, AddressKind::Particular);

    assert_eq!(converted_address.id, id);
    assert_eq!(converted_address.post_box, Some("BP 1234".to_string()));
    assert_eq!(converted_address.post_code, Some("31000".to_string()));
    assert_eq!(converted_address.town_name, Some("TOULOUSE".to_string()));
    assert_eq!(converted_address.country, Some("FR".to_string()));
}

#[test]
fn company_with_multiple_floors() {
    let repo: Box<dyn AddressRepository> = Box::new(InMemoryAddressRepository::new());
    let service = AddressService::new(repo);

    let id = Uuid::new_v4().to_string();
    let address = FrenchAddress {
        id: id.clone(),
        line1: Some("IBM FRANCE".to_string()),
        line2: Some("Head Office".to_string()),
        line3: Some("Tour Pacific 5th and 6th Floors".to_string()),
        line4: Some("11 COURS VALMY".to_string()),
        line5: None,
        line6: Some("92800 PUTEAUX".to_string()),
        line7: Some("France".to_string()),
    };

    let converted_address = service.convert_to_iso(&address, AddressKind::Company);

    assert_eq!(converted_address.id, id);
    assert_eq!(
        converted_address.department,
        Some("Head Office".to_string())
    );
    assert_eq!(
        converted_address.floor,
        Some("Tour Pacific 5th and 6th Floors".to_string())
    );
    assert_eq!(converted_address.building_name, None);
    assert_eq!(
        converted_address.street_name,
        Some("COURS VALMY".to_string())
    );
    assert_eq!(converted_address.building_number, Some("11".to_string()));
    assert_eq!(converted_address.post_code, Some("92800".to_string()));
    assert_eq!(converted_address.town_name, Some("PUTEAUX".to_string()));
    assert_eq!(converted_address.country, Some("FR".to_string()));
}
