use crate::domain::models::ISO20022Address;
use crate::domain::repository::{AddressRepository, ReadAddressRepository};
use mongodb::{
    bson::doc,
    options::ClientOptions,
    sync::{Client, Collection},
};

#[derive(Clone)]
pub struct MongoAddressRepository {
    collection: Collection<ISO20022Address>,
}

impl MongoAddressRepository {
    pub fn new(uri: &str, db_name: &str, coll_name: &str) -> Result<Self, String> {
        // parse connection options (blocking if needed):
        let client_options = ClientOptions::parse(uri).map_err(|e| e.to_string())?;
        // build synchronous Mongo client
        let client = Client::with_options(client_options).map_err(|e| e.to_string())?;
        let db = client.database(db_name);
        let collection = db.collection::<ISO20022Address>(coll_name);

        Ok(Self { collection })
    }
}

impl AddressRepository for MongoAddressRepository {
    fn save(&mut self, address: ISO20022Address) -> Result<(), String> {
        self.collection
            .insert_one(address, None)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn update(&mut self, address: ISO20022Address) -> Result<(), String> {
        let filter = doc! { "id": &address.id };
        let result = self
            .collection
            .replace_one(filter, address.clone(), None)
            .map_err(|e| e.to_string())?;
        if result.matched_count == 0 {
            return Err(format!("No document found with id={}", address.id));
        }
        Ok(())
    }

    fn delete(&mut self, address_id: &str) -> Result<(), String> {
        let filter = doc! { "id": address_id };
        let result = self
            .collection
            .delete_one(filter, None)
            .map_err(|e| e.to_string())?;
        if result.deleted_count == 0 {
            return Err(format!("No document found with id={}", address_id));
        }
        Ok(())
    }
}

impl ReadAddressRepository for MongoAddressRepository {
    fn find_by_id(&self, address_id: &str) -> Option<ISO20022Address> {
        let filter = doc! { "id": address_id };
        self.collection.find_one(filter, None).ok().flatten() // return None if error or if not found
    }

    fn find_all(&self) -> Vec<ISO20022Address> {
        let cursor = match self.collection.find(None, None) {
            Ok(cur) => cur,
            Err(_) => return vec![],
        };
        cursor.filter_map(|res| res.ok()).collect()
    }
}
