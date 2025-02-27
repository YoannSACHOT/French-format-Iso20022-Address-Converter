use crate::domain::models::{FrenchAddress, ISO20022Address};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum AddressKind {
    #[default]
    Particular,
    Company,
}

fn parse_postal_code_and_town(s: &str) -> (Option<String>, Option<String>) {
    let s = s.trim();
    let re = Regex::new(r"^(\d{5})(?:\s+(.+))?$").unwrap();
    if let Some(caps) = re.captures(s) {
        let postal_code = caps.get(1).map(|m| m.as_str().to_string());
        let town = caps.get(2).map(|m| m.as_str().to_string());
        (postal_code, town)
    } else {
        (None, None)
    }
}

fn process_street(line4: &Option<String>) -> (Option<String>, Option<String>) {
    if let Some(line) = line4 {
        let mut parts = line.split_whitespace();
        if let Some(first_token) = parts.next() {
            if first_token.chars().next().map_or(false, |c| c.is_digit(10)) {
                let building_number = Some(first_token.to_string());
                let street_name = parts.collect::<Vec<&str>>().join(" ");
                let street_name = if street_name.is_empty() {
                    None
                } else {
                    Some(street_name)
                };
                return (building_number, street_name);
            }
        }
        return (None, Some(line.clone()));
    }
    (None, None)
}

fn process_common_fields(address: &FrenchAddress, iso: &mut ISO20022Address) {
    iso.post_box = address.line5.clone();

    if let Some(ref line6) = address.line6 {
        let (postal_code, town) = parse_postal_code_and_town(line6);
        iso.post_code = postal_code;
        iso.town_name = town;
    }

    if let Some(ref line7) = address.line7 {
        iso.country = Some(if line7.trim().eq_ignore_ascii_case("france") {
            "FR".to_string()
        } else {
            line7.clone()
        });
    }

    let (building_number, street_name) = process_street(&address.line4);
    iso.building_number = building_number;
    iso.street_name = street_name;
}

pub fn convert_to_iso(address: &FrenchAddress, kind: AddressKind) -> ISO20022Address {
    println!("DEBUG - AddressKind: {:?}", kind);

    let mut iso = ISO20022Address::default();
    iso.id = address.id.clone();
    iso.recipient_name = address.line1.clone();
    iso.kind = kind; // 🚨 Si ici `kind` est Particular, l'erreur vient de l'appel

    println!("DEBUG - ISO20022Address before update: {:#?}", iso);

    process_common_fields(address, &mut iso);

    match kind {
        AddressKind::Company => {
            iso.department = address.line2.clone();
            iso.floor = address.line3.clone();
        }
        AddressKind::Particular => {
            iso.room = address.line2.clone();
            iso.floor = address.line3.clone();
        }
    }

    println!("DEBUG - ISO20022Address after update: {:#?}", iso);
    iso
}


pub fn convert_to_french(iso: &ISO20022Address) -> FrenchAddress {
    let mut french = FrenchAddress {
        id: iso.id.clone(),
        line1: iso.recipient_name.clone(),
        line2: None, // On corrigera ce champ après
        line3: iso.floor.clone(),
        line4: iso
            .building_number
            .as_ref()
            .map_or(iso.street_name.clone(), |num| {
                Some(format!(
                    "{} {}",
                    num,
                    iso.street_name.as_deref().unwrap_or("")
                ))
            }),
        line5: iso.post_box.clone(),
        line6: iso
            .post_code
            .as_ref()
            .map_or(iso.town_name.clone(), |code| {
                Some(format!(
                    "{} {}",
                    code,
                    iso.town_name.as_deref().unwrap_or("")
                ))
            }),
        line7: iso
            .country
            .clone()
            .map(|c| if c == "FR" { "France".to_string() } else { c }),
    };

    match iso.kind {
        AddressKind::Company => {
            french.line2 = iso
                .department
                .clone()
                .or_else(|| iso.sub_department.clone());
        }
        AddressKind::Particular => {
            french.line2 = iso.room.clone();
        }
    }

    french
}

#[cfg(test)]
mod tests {
    use super::*;

    //////////////////////////////////////CONVERT TO ISO//////////////////////////////////////
    #[test]
    fn test_parse_postal_code_and_town() {
        assert_eq!(
            parse_postal_code_and_town("33506 LIBOURNE CEDEX"),
            (
                Some("33506".to_string()),
                Some("LIBOURNE CEDEX".to_string())
            )
        );

        assert_eq!(
            parse_postal_code_and_town("75000 PARIS"),
            (Some("75000".to_string()), Some("PARIS".to_string()))
        );

        assert_eq!(parse_postal_code_and_town("INVALID"), (None, None));
    }

    #[test]
    fn test_process_street() {
        assert_eq!(
            process_street(&Some("10 Rue de la Paix".to_string())),
            (Some("10".to_string()), Some("Rue de la Paix".to_string()))
        );

        assert_eq!(
            process_street(&Some("Boulevard Haussmann".to_string())),
            (None, Some("Boulevard Haussmann".to_string()))
        );

        assert_eq!(process_street(&None), (None, None));
    }

    #[test]
    fn test_process_common_fields() {
        let address = FrenchAddress {
            id: "123".to_string(),
            line1: None,
            line2: None,
            line3: None,
            line4: Some("10 Rue de la Paix".to_string()),
            line5: Some("BP 40122".to_string()),
            line6: Some("75000 PARIS".to_string()),
            line7: Some("France".to_string()),
        };

        let mut iso = ISO20022Address::default();
        process_common_fields(&address, &mut iso);

        assert_eq!(iso.building_number, Some("10".to_string()));
        assert_eq!(iso.street_name, Some("Rue de la Paix".to_string()));
        assert_eq!(iso.post_box, Some("BP 40122".to_string()));
        assert_eq!(iso.post_code, Some("75000".to_string()));
        assert_eq!(iso.town_name, Some("PARIS".to_string()));
        assert_eq!(iso.country, Some("FR".to_string()));
    }

    #[test]
    fn test_convert_to_iso_company() {
        let address = FrenchAddress {
            id: "123".to_string(),
            line1: None,
            line2: Some("Finance Department".to_string()),
            line3: Some("5th Floor".to_string()),
            line4: Some("1 Avenue de l'Opéra".to_string()),
            line5: None,
            line6: Some("75001 PARIS".to_string()),
            line7: Some("France".to_string()),
        };

        let iso = convert_to_iso(&address, AddressKind::Company);

        assert_eq!(iso.id, "123");
        assert_eq!(iso.department, Some("Finance Department".to_string()));
        assert_eq!(iso.floor, Some("5th Floor".to_string()));
        assert_eq!(iso.building_number, Some("1".to_string()));
        assert_eq!(iso.street_name, Some("Avenue de l'Opéra".to_string()));
        assert_eq!(iso.post_code, Some("75001".to_string()));
        assert_eq!(iso.town_name, Some("PARIS".to_string()));
        assert_eq!(iso.country, Some("FR".to_string()));
    }

    #[test]
    fn test_convert_to_iso_particular() {
        let address = FrenchAddress {
            id: "456".to_string(),
            line1: Some("Jean Dupont".to_string()),
            line2: Some("Apt. 12B".to_string()),
            line3: Some("3rd Floor".to_string()),
            line4: Some("15 Rue des Lilas".to_string()),
            line5: None,
            line6: Some("69000 LYON".to_string()),
            line7: Some("France".to_string()),
        };

        let iso = convert_to_iso(&address, AddressKind::Particular);

        assert_eq!(iso.id, "456");
        assert_eq!(iso.room, Some("Apt. 12B".to_string()));
        assert_eq!(iso.floor, Some("3rd Floor".to_string()));
        assert_eq!(iso.building_number, Some("15".to_string()));
        assert_eq!(iso.street_name, Some("Rue des Lilas".to_string()));
        assert_eq!(iso.post_code, Some("69000".to_string()));
        assert_eq!(iso.town_name, Some("LYON".to_string()));
        assert_eq!(iso.country, Some("FR".to_string()));
    }
    //////////////////////////////////////CONVERT TO FRENCH//////////////////////////////////////
    #[test]
    fn test_convert_to_french_company() {
        let iso = ISO20022Address {
            id: "123".to_string(),
            recipient_name: Some("DURAND SA".to_string()),
            kind: AddressKind::Company,
            department: Some("Finance Department".to_string()),
            floor: Some("5th Floor".to_string()),
            building_number: Some("1".to_string()),
            street_name: Some("Avenue de l'Opéra".to_string()),
            post_box: None,
            post_code: Some("75001".to_string()),
            town_name: Some("PARIS".to_string()),
            country: Some("FR".to_string()),
            ..Default::default()
        };

        let french = convert_to_french(&iso);

        assert_eq!(french.id, "123");
        assert_eq!(french.line1, Some("DURAND SA".to_string()));
        assert_eq!(french.line2, Some("Finance Department".to_string()));
        assert_eq!(french.line3, Some("5th Floor".to_string()));
        assert_eq!(french.line4, Some("1 Avenue de l'Opéra".to_string()));
        assert_eq!(french.line6, Some("75001 PARIS".to_string()));
        assert_eq!(french.line7, Some("France".to_string()));
    }

    #[test]
    fn test_convert_to_french_particular() {
        let iso = ISO20022Address {
            id: "456".to_string(),
            recipient_name: Some("Jean Dupont".to_string()),
            kind: AddressKind::Particular,
            room: Some("Apt. 12B".to_string()),
            floor: Some("3rd Floor".to_string()),
            building_number: Some("15".to_string()),
            street_name: Some("Rue des Lilas".to_string()),
            post_code: Some("69000".to_string()),
            town_name: Some("LYON".to_string()),
            country: Some("FR".to_string()),
            ..Default::default()
        };

        let french = convert_to_french(&iso);

        assert_eq!(french.id, "456");
        assert_eq!(french.line1, Some("Jean Dupont".to_string()));
        assert_eq!(french.line2, Some("Apt. 12B".to_string()));
        assert_eq!(french.line3, Some("3rd Floor".to_string()));
        assert_eq!(french.line4, Some("15 Rue des Lilas".to_string()));
        assert_eq!(french.line6, Some("69000 LYON".to_string()));
        assert_eq!(french.line7, Some("France".to_string()));
    }

    #[test]
    fn test_convert_to_french_company_with_department() {
        let iso = ISO20022Address {
            id: "123".to_string(),
            kind: AddressKind::Company,
            recipient_name: Some("DURAND SA".to_string()),
            department: Some("Purchasing Department".to_string()), // ✅ Ce champ doit être converti en `line2`
            floor: Some("Industrial Zone".to_string()),
            building_number: Some("22BIS".to_string()),
            street_name: Some("Rue des Fleurs".to_string()),
            post_box: Some("BP 40122".to_string()),
            post_code: Some("33506".to_string()),
            town_name: Some("LIBOURNE CEDEX".to_string()),
            country: Some("FR".to_string()),
            ..Default::default()
        };

        let french = convert_to_french(&iso);

        assert_eq!(french.line1, Some("DURAND SA".to_string()));
        assert_eq!(french.line2, Some("Purchasing Department".to_string())); // ✅ Vérification
        assert_eq!(french.line3, Some("Industrial Zone".to_string()));
        assert_eq!(french.line4, Some("22BIS Rue des Fleurs".to_string()));
        assert_eq!(french.line5, Some("BP 40122".to_string()));
        assert_eq!(french.line6, Some("33506 LIBOURNE CEDEX".to_string()));
        assert_eq!(french.line7, Some("France".to_string()));
    }
}
