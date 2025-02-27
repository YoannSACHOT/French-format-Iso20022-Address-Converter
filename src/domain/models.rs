use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrenchAddress {
    pub id: String,
    pub line1: Option<String>,
    pub line2: Option<String>, // Identité du destinataire et/ou Service Max 38 caractères
    pub line3: Option<String>, // Identification du point géographique (Entrée, immeuble, bâtiment,résidence, zone industrielle) Max 38 caractères
    pub line4: Option<String>, // N° et Libellé de la voie Max 38 caractères
    pub line5: Option<String>, // MENTIONS SPECIALES DE DISTRIBUTION (BP, Tri service arrivée…) et COMMUNE D'IMPLANTATION DE L'ENTREPRISE (Si différente du bureau distributeur CEDEX) MAX 38 caractères
    pub line6: Option<String>, //CODE POSTAL et LOCALITE DE DESTINATION ou CODE CEDEX et BUREAU DISTRIBUTEUR CEDEX et PAYS
    pub line7: Option<String>, // nom du Pays
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ISO20022Address {
    pub id: String,
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
