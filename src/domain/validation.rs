use crate::domain::models::{FrenchAddress, ISO20022Address};
use regex::Regex;

#[derive(Debug)]
pub enum ValidationError {
    EmptyField {
        field: &'static str,
    },
    TooLong {
        field: &'static str,
        max_len: usize,
        actual_len: usize,
    },
    InvalidPostalCode {
        value: String,
    },
    InvalidCountryCode {
        value: String,
    },
}

fn check_line_length(
    val: &Option<String>,
    field: &'static str,
    max_len: usize,
) -> Result<(), ValidationError> {
    if let Some(v) = val {
        if v.len() > max_len {
            return Err(ValidationError::TooLong {
                field,
                max_len,
                actual_len: v.len(),
            });
        }
    }
    Ok(())
}

pub fn validate_french_address(addr: &FrenchAddress) -> Result<(), ValidationError> {
    check_line_length(&addr.line1, "line1", 38)?;
    check_line_length(&addr.line2, "line2", 38)?;
    check_line_length(&addr.line3, "line3", 38)?;
    check_line_length(&addr.line4, "line4", 38)?;
    check_line_length(&addr.line5, "line5", 38)?;
    check_line_length(&addr.line6, "line6", 38)?;
    check_line_length(&addr.line7, "line7", 38)?;

    //Check that line6 is present and looks like a 5-digit code plus a city (optional)
    if let Some(line6) = &addr.line6 {
        let re = Regex::new(r"^(\d{5})(?:\s+(.+))?$").unwrap();
        if !re.is_match(line6.trim()) {
            return Err(ValidationError::InvalidPostalCode {
                value: line6.to_string(),
            });
        }
    } else {
        return Err(ValidationError::EmptyField { field: "line6" });
    }

    if let Some(l7) = &addr.line7 {
        if l7.trim().is_empty() {
            return Err(ValidationError::EmptyField { field: "line7" });
        }
    } else {
        return Err(ValidationError::EmptyField { field: "line7" });
    }

    Ok(())
}

pub fn validate_iso20022_address(addr: &ISO20022Address) -> Result<(), ValidationError> {
    check_line_length(&addr.street_name, "street_name", 70)?;
    check_line_length(&addr.building_number, "building_number", 16)?;
    check_line_length(&addr.building_name, "building_name", 35)?;
    check_line_length(&addr.post_box, "post_box", 16)?;
    check_line_length(&addr.floor, "floor", 70)?;
    check_line_length(&addr.room, "room", 70)?;
    check_line_length(&addr.post_code, "post_code", 16)?;
    check_line_length(&addr.town_name, "town_name", 35)?;
    check_line_length(&addr.town_location_name, "town_location_name", 35)?;
    check_line_length(&addr.department, "department", 70)?;
    check_line_length(&addr.sub_department, "sub_department", 70)?;
    check_line_length(&addr.district_name, "district_name", 35)?;
    check_line_length(&addr.country_sub_division, "country_sub_division", 35)?;

    if let Some(ctry) = &addr.country {
        if ctry.len() != 2 {
            return Err(ValidationError::InvalidCountryCode {
                value: ctry.clone(),
            });
        }
    }

    Ok(())
}
