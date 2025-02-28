use clap::Parser;
use fraddris020022::cli;
use fraddris020022::cli::commands::Cli;
use fraddris020022::domain::repository::AddressRepository;
use fraddris020022::infrastructure::{
    file_repository::FileBasedAddressRepository, in_memory_repository::InMemoryAddressRepository,
    postgresql_repository::PostgresAddressRepository,
};
use std::env;

fn main() {
    let cli = Cli::parse();

    let repo: Box<dyn AddressRepository> = match env::var("SELECT_REPO")
        .unwrap_or_else(|_| "file".to_string())
        .as_str()
    {
        "postgres" => {
            let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be defined !");
            Box::new(
                PostgresAddressRepository::new(&db_url).expect("PostgreSQL connection error !"),
            )
        }
        "inmemory" => Box::new(InMemoryAddressRepository::new()),
        _ => Box::new(FileBasedAddressRepository::new(
            "addresses.json".to_string(),
        )),
    };

    cli::commands::run(cli, repo);
}
