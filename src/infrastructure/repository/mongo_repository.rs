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
        // Work around a MongoDB driver quirk where `tlsInsecure=false` in the URI
        // may inadvertently disable certificate validation. We sanitize the URI by
        // removing `tlsInsecure=false` and explicitly enforcing valid certificates
        // and hostnames.
        let sanitized = sanitize_tls_insecure_false(uri);
        let client = Client::with_uri_str(&sanitized).map_err(|e| e.to_string())?;
        let db = client.database(db_name);
        let collection = db.collection::<ISO20022Address>(coll_name);
        Ok(Self { collection })
    }
}

/// Remove `tlsInsecure=false` from the connection string and ensure strict TLS
/// verification via `tlsAllowInvalidCertificates=false` and
/// `tlsAllowInvalidHostnames=false`.
fn sanitize_tls_insecure_false(uri: &str) -> String {
    // Split URI into base and query parts.
    let mut parts = uri.splitn(2, '?');
    let base = parts.next().unwrap_or("");
    let query = parts.next();

    // Parse and filter query params (simple, case-insensitive for the key).
    let mut params: Vec<(String, String)> = vec![];
    if let Some(q) = query {
        for pair in q.split('&') {
            if pair.is_empty() { continue; }
            let mut kv = pair.splitn(2, '=');
            let k = kv.next().unwrap_or("");
            let v = kv.next().unwrap_or("");
            // Drop tlsInsecure=false (case-insensitive key, value exactly "false").
            if k.eq_ignore_ascii_case("tlsInsecure") && v.eq_ignore_ascii_case("false") {
                continue;
            }
            params.push((k.to_string(), v.to_string()));
        }
    }

    // Ensure strict validation flags are present (avoid duplicates if already set).
    fn has_key(params: &[(String, String)], key: &str) -> bool {
        params.iter().any(|(k, _)| k.eq_ignore_ascii_case(key))
    }

    if !has_key(&params, "tlsAllowInvalidCertificates") {
        params.push(("tlsAllowInvalidCertificates".into(), "false".into()));
    }
    if !has_key(&params, "tlsAllowInvalidHostnames") {
        params.push(("tlsAllowInvalidHostnames".into(), "false".into()));
    }

    // Rebuild the URI.
    if params.is_empty() {
        base.to_string()
    } else {
        let q = params
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        format!("{}?{}", base, q)
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

#[cfg(test)]
mod tests {
    use super::sanitize_tls_insecure_false;

    #[test]
    fn sanitize_removes_tls_insecure_false_and_adds_strict_flags() {
        let uri = "mongodb://user:pass@host:27017/db?authSource=admin&tls=true&tlsInsecure=false";
        let out = sanitize_tls_insecure_false(uri);
        assert!(!out.to_lowercase().contains("tlsinsecure=false"));
        assert!(out.contains("tlsAllowInvalidCertificates=false"));
        assert!(out.contains("tlsAllowInvalidHostnames=false"));
        assert!(out.starts_with("mongodb://user:pass@host:27017/db?"));
    }

    #[test]
    fn sanitize_preserves_existing_strict_flags() {
        let uri = "mongodb://h/db?tls=true&tlsAllowInvalidCertificates=false&tlsAllowInvalidHostnames=false&tlsInsecure=false";
        let out = sanitize_tls_insecure_false(uri);
        // Should not duplicate flags
        let count_cert = out.match_indices("tlsAllowInvalidCertificates=").count();
        let count_host = out.match_indices("tlsAllowInvalidHostnames=").count();
        assert_eq!(count_cert, 1);
        assert_eq!(count_host, 1);
        assert!(!out.to_lowercase().contains("tlsinsecure=false"));
    }

    #[test]
    fn sanitize_adds_query_when_absent() {
        let uri = "mongodb+srv://cluster0.example.com/db";
        let out = sanitize_tls_insecure_false(uri);
        assert!(out.starts_with("mongodb+srv://cluster0.example.com/db?"));
        assert!(out.contains("tlsAllowInvalidCertificates=false"));
        assert!(out.contains("tlsAllowInvalidHostnames=false"));
    }
}
