use actix_web::{App, HttpServer, web};
use fraddriso20022::application::address_service::AddressService;
use fraddriso20022::domain::repository::AddressRepository;
use fraddriso20022::infrastructure::app_state::AppState;
use fraddriso20022::infrastructure::file_repository::FileBasedAddressRepository;
use fraddriso20022::infrastructure::in_memory_repository::InMemoryAddressRepository;
use fraddriso20022::infrastructure::mongo_repository::MongoAddressRepository;
use fraddriso20022::infrastructure::rest_controller::configure_routes;
use std::env;
use std::sync::Mutex;

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

    let service = AddressService::new(repo);
    let shared_state = web::Data::new(AppState {
        service: Mutex::new(service),
    });

    println!("Server running at http://127.0.0.1:8080 ...");

    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .configure(configure_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
