use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FrenchAddress {
    pub id: String,
    pub line1: Option<String>, // Name of the company or individual (private person)
    pub line2: Option<String>, // Recipient's identity and/or department (Max 38 characters)
    pub line3: Option<String>, // Geographic identification (Entrance, building, residence, industrial area) (Max 38 characters)
    pub line4: Option<String>, // Street number and name (Max 38 characters)
    pub line5: Option<String>, // Special distribution mentions (PO Box, arrival sorting service…) and COMPANY LOCATION (if different from the main distribution office) (Max 38 characters)
    pub line6: Option<String>, // POSTAL CODE and DESTINATION CITY or CEDEX CODE and CEDEX DISTRIBUTION OFFICE and COUNTRY
    pub line7: Option<String>, // Country name
}

#[derive(Default, Clone)]
pub struct FrenchAddressBuilder {
    id: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    line3: Option<String>,
    line4: Option<String>,
    line5: Option<String>,
    line6: Option<String>,
    line7: Option<String>,
}

impl FrenchAddressBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            line1: None,
            line2: None,
            line3: None,
            line4: None,
            line5: None,
            line6: None,
            line7: None,
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn line1(mut self, line1: Option<String>) -> Self {
        self.line1 = line1;
        self
    }

    pub fn line2(mut self, line2: Option<String>) -> Self {
        self.line2 = line2;
        self
    }

    pub fn line3(mut self, line3: Option<String>) -> Self {
        self.line3 = line3;
        self
    }

    pub fn line4(mut self, line4: Option<String>) -> Self {
        self.line4 = line4;
        self
    }

    pub fn line5(mut self, line5: Option<String>) -> Self {
        self.line5 = line5;
        self
    }

    pub fn line6(mut self, line6: Option<String>) -> Self {
        self.line6 = line6;
        self
    }

    pub fn line7(mut self, line7: Option<String>) -> Self {
        self.line7 = line7;
        self
    }

    pub fn build(self) -> Result<FrenchAddress, String> {
        let addr = FrenchAddress {
            id: self.id.unwrap_or_default(),
            line1: self.line1,
            line2: self.line2,
            line3: self.line3,
            line4: self.line4,
            line5: self.line5,
            line6: self.line6,
            line7: self.line7,
        };

        // Call domain validation logic:
        if let Err(e) = crate::domain::validation::validate_french_address(&addr) {
            return Err(format!("Validation error: {:?}", e));
        }

        Ok(addr)
    }
}

#[derive(Builder, Debug, Serialize, Deserialize, Clone, Default)]
#[builder(default)]
pub struct ISO20022Address {
    pub id: String,
    pub recipient_name: Option<String>, //Name of a person or organization.
    pub kind: AddressKind,              //enum Company or Particular (default)
    pub department: Option<String>, //Identification of a division of a large organization or building.
    pub sub_department: Option<String>, //Identification of a subdivision of a large organization or building.
    pub building_name: Option<String>,  //Name of a building or house
    pub floor: Option<String>,          //Floor or storey within a building.
    pub room: Option<String>,           // Building room number
    pub street_name: Option<String>,    // Name of a street or thoroughfare.
    pub building_number: Option<String>, // Number that identifies the position of a building on a street.
    pub post_box: Option<String>, // Numbered box in a post office, assigned to a person or organization, where letters are kept until called for.
    pub town_location_name: Option<String>, // Specific location within the town.
    pub post_code: Option<String>, // Identifier consisting of letters and/or numbers that is added to a postal address to assist the sorting of mail.
    pub town_name: Option<String>, // Name of a built-up area, with defined boundaries, and a local government.
    pub country: Option<String>,   // Nation with its own government.
    pub district_name: Option<String>, //Identifies a subdivision within a country sub-division.
    pub country_sub_division: Option<String>, // Identifies a subdivision of a country such as state, region, county.
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum AddressKind {
    #[default]
    Particular,
    Company,
}
