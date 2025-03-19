use crate::domain::models::ISO20022Address;
use crate::domain::repository::{AddressRepository, ReadAddressRepository};
use mongodb::{
    bson::doc,
    sync::{Client, Collection},
};

#[derive(Clone)]
pub struct MongoAddressRepository {
    collection: Collection<ISO20022Address>,
}

impl MongoAddressRepository {
    pub fn new(uri: &str, db_name: &str, coll_name: &str) -> Result<Self, String> {
        let client = Client::with_uri_str(uri).map_err(|e| e.to_string())?;
        let db = client.database(db_name);
        let collection = db.collection::<ISO20022Address>(coll_name);
        Ok(Self { collection })
    }
}

impl AddressRepository for MongoAddressRepository {
    fn save(&mut self, address: ISO20022Address) -> Result<(), String> {
        self.collection
            .insert_one(address)
            .run()
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn update(&mut self, address: ISO20022Address) -> Result<(), String> {
        let filter = doc! { "id": &address.id };
        self.collection
            .replace_one(filter, address)
            .run()
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn delete(&mut self, address_id: &str) -> Result<(), String> {
        let filter = doc! { "id": address_id };
        self.collection
            .delete_one(filter)
            .run()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl ReadAddressRepository for MongoAddressRepository {
    fn find_by_id(&self, address_id: &str) -> Option<ISO20022Address> {
        let filter = doc! { "id": address_id };
        self.collection.find_one(filter).run().unwrap_or_else(|_| None)
    }

    fn find_all(&self) -> Vec<ISO20022Address> {
        let cursor = match self.collection.find(doc! {}).run() {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        cursor
            .map(|doc_result| doc_result.unwrap_or_default())
            .collect()
    }
}
