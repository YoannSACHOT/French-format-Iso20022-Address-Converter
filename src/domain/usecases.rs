use regex::Regex;
use crate::domain::models::{FrenchAddress, ISO20022Address};

/// Type d'adresse pour déterminer le mapping spécifique selon la spec CFONB.
#[derive(Debug, Clone, Copy)]
pub enum AddressKind {
    Company,
    Particular,
}

/// Extrait le code postal (5 chiffres) et le reste comme localité depuis une chaîne.
/// Exemple : "33506 LIBOURNE CEDEX" -> (Some("33506"), Some("LIBOURNE CEDEX"))
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

/// Convertit une adresse française en adresse ISO 20022 en respectant les règles CFONB.
pub fn convert_to_iso(address: &FrenchAddress, kind: AddressKind) -> ISO20022Address {
    let mut iso = ISO20022Address::default();
    iso.id = address.id.clone(); // ✅ Conservation de l'ID original

    match kind {
        AddressKind::Company => {
            iso.department = address.line2.clone();
            iso.floor = address.line3.clone();

            if let Some(ref line4) = address.line4 {
                let mut parts = line4.split_whitespace();
                if let Some(first_token) = parts.next() {
                    if first_token.chars().next().map_or(false, |c| c.is_digit(10)) {
                        iso.building_number = Some(first_token.to_string());
                        let street_name = parts.collect::<Vec<&str>>().join(" ");
                        if !street_name.is_empty() {
                            iso.street_name = Some(street_name);
                        }
                    } else {
                        iso.street_name = Some(line4.clone());
                    }
                }
            }

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
        }
        AddressKind::Particular => {
            iso.room = address.line2.clone();
            iso.floor = address.line3.clone();

            if let Some(ref line4) = address.line4 {
                let mut parts = line4.split_whitespace();
                if let Some(first_token) = parts.next() {
                    if first_token.chars().next().map_or(false, |c| c.is_digit(10)) {
                        iso.building_number = Some(first_token.to_string());
                        let street_name = parts.collect::<Vec<&str>>().join(" ");
                        if !street_name.is_empty() {
                            iso.street_name = Some(street_name);
                        }
                    } else {
                        iso.street_name = Some(line4.clone());
                    }
                }
            }

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
        }
    }

    iso
}
