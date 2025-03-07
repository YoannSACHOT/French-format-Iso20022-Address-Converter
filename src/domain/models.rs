use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ISO20022Address {
    pub id: String,
    pub recipient_name : Option<String>,      //Name of a person or organization.
    pub kind: AddressKind,                    //enum Company or Particular (default)
    pub department: Option<String>,           //Identification of a division of a large organization or building.
    pub sub_department: Option<String>,       //Identification of a subdivision of a large organization or building.
    pub building_name: Option<String>,        //Name of a building or house
    pub floor: Option<String>,                //Floor or storey within a building.
    pub room: Option<String>,                 // Building room number
    pub street_name: Option<String>,          // Name of a street or thoroughfare.
    pub building_number: Option<String>,      // Number that identifies the position of a building on a street.
    pub post_box: Option<String>,             // Numbered box in a post office, assigned to a person or organization, where letters are kept until called for.
    pub town_location_name: Option<String>,   // Specific location within the town.
    pub post_code: Option<String>,            // Identifier consisting of letters and/or numbers that is added to a postal address to assist the sorting of mail.
    pub town_name: Option<String>,            // Name of a built-up area, with defined boundaries, and a local government.
    pub country: Option<String>,              // Nation with its own government.
    pub district_name: Option<String>,        //Identifies a subdivision within a country sub-division.
    pub country_sub_division: Option<String>, // Identifies a subdivision of a country such as state, region, county.
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum AddressKind {
    #[default]
    Particular,
    Company,
}