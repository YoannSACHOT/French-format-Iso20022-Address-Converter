use actix_web::{web, App, HttpServer};
use fraddriso20022::application::command::address_command_service::AddressCommandService;
use fraddriso20022::application::query::address_query_service::AddressQueryService;
use fraddriso20022::domain::repository::{AddressRepository, ReadAddressRepository};
use fraddriso20022::infrastructure::app_state::AppState;
use fraddriso20022::infrastructure::controller::rest_controller::configure_routes;
use fraddriso20022::infrastructure::repository::file_repository::FileBasedAddressRepository;
use fraddriso20022::infrastructure::repository::in_memory_repository::InMemoryAddressRepository;
use fraddriso20022::infrastructure::repository::mongo_repository::MongoAddressRepository;
use std::env;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let selected_repo = env::var("SELECT_REPO").unwrap_or_else(|_| "file".to_string());

    let (write_repo, read_repo): (
        Box<dyn AddressRepository + Send>,
        Box<dyn ReadAddressRepository + Send>,
    ) = match selected_repo.as_str() {
        "inmemory" => {
            let repo_inmem = InMemoryAddressRepository::new();
            (Box::new(repo_inmem.clone()), Box::new(repo_inmem))
        }
        "mongo" | "mongodb" => {
            let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be defined!");
            let db_name = env::var("MONGO_DB_NAME").unwrap_or_else(|_| "addresses_db".into());
            let coll_name = env::var("MONGO_DB_COLLECTION").unwrap_or_else(|_| "addresses".into());

            let repo_mongo = MongoAddressRepository::new(&mongo_uri, &db_name, &coll_name)
                .expect("Cannot connect to MongoDB!");
            (Box::new(repo_mongo.clone()), Box::new(repo_mongo))
        }
        _ => {
            let repo_file = FileBasedAddressRepository::new("addresses.json".to_string());
            (Box::new(repo_file.clone()), Box::new(repo_file))
        }
    };

    let command_service = AddressCommandService::new(write_repo);
    let query_service = AddressQueryService::new(read_repo);

    let shared_state = web::Data::new(AppState {
        command_service: Mutex::new(command_service),
        query_service: Mutex::new(query_service),
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
