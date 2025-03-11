use clap::Parser;
use fraddris020022::cli;
use fraddris020022::cli::commands::Cli;
use fraddris020022::domain::repository::AddressRepository;
use fraddris020022::infrastructure::{
    file_repository::FileBasedAddressRepository, in_memory_repository::InMemoryAddressRepository,
    mongo_repository::MongoAddressRepository,
};
use std::env;

fn main() {
    let cli = Cli::parse();

    let repo: Box<dyn AddressRepository> = match env::var("SELECT_REPO")
        .unwrap_or_else(|_| "file".to_string())
        .as_str()
    {
        "inmemory" => Box::new(InMemoryAddressRepository::new()),
        "mongo" | "mongodb" => {
            let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be defined!");
            // Use whatever default DB / collection name you like:
            let db_name = env::var("MONGO_DB_NAME").unwrap_or_else(|_| "addresses_db".into());
            let collection_name =
                env::var("MONGO_DB_COLLECTION").unwrap_or_else(|_| "addresses".into());

            Box::new(
                MongoAddressRepository::new(&mongo_uri, &db_name, &collection_name)
                    .expect("MongoDB connection error!"),
            )
        }
        _ => Box::new(FileBasedAddressRepository::new(
            "addresses.json".to_string(),
        )),
    };

    cli::commands::run(cli, repo);
}
