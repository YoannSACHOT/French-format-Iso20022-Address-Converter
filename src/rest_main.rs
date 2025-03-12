use crate::application::address_service::AddressService;
use crate::domain::repository::AddressRepository;
use crate::infrastructure::{
    file_repository::FileBasedAddressRepository,
    in_memory_repository::InMemoryAddressRepository,
    mongo_repository::MongoAddressRepository,
    rest_controller::configure_routes,
};

use actix_web::{App, HttpServer, web};
use std::env;
use std::sync::Mutex;

pub struct AppState {
    pub service: Mutex<AddressService>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo: Box<dyn AddressRepository + Send> = match env::var("SELECT_REPO")
        .unwrap_or_else(|_| "file".to_string())
        .as_str()
    {
        "inmemory" => Box::new(InMemoryAddressRepository::new()),
        "mongo" | "mongodb" => {
            let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be defined!");
            let db_name = env::var("MONGO_DB_NAME").unwrap_or_else(|_| "addresses_db".into());
            let coll_name = env::var("MONGO_DB_COLLECTION").unwrap_or_else(|_| "addresses".into());
            Box::new(
                MongoAddressRepository::new(&mongo_uri, &db_name, &coll_name)
                    .expect("Cannot connect to MongoDB!"),
            )
        }
        _ => Box::new(FileBasedAddressRepository::new(
            "addresses.json".to_string(),
        )),
    };

    // Construire un AddressService qu’on réutilisera pour toutes les requêtes
    let service = AddressService::new(repo);

    // State Actix : on met le service dans un Mutex,
    // afin de pouvoir le muter en toute sécurité s’il le faut (ex: .add_address, .delete_address...)
    let shared_state = web::Data::new(AppState {
        service: Mutex::new(service),
    });

    println!("Server running at http://127.0.0.1:8080 ...");

    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
