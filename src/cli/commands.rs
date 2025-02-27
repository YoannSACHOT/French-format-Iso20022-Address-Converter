use crate::application::address_service::AddressService;
use crate::domain::models::FrenchAddress;
use crate::domain::repository::AddressRepository;
use crate::domain::usecases::AddressKind;
use crate::infrastructure::file_repository::FileBasedAddressRepository;
use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[command(
    name = "postal_converter",
    version = "1.0",
    about = "Convert French addresses to ISO 20022 format"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Convert {
        #[arg(short, long)]
        id: String,
    },
    Add {
        #[arg(short, long)]
        kind: String,
        #[arg(short = 'a', long)]
        line1: Option<String>,
        #[arg(short = 'b', long)]
        line2: Option<String>,
        #[arg(short = 'c', long)]
        line3: Option<String>,
        #[arg(short = 'd', long)]
        line4: Option<String>,
        #[arg(short = 'e', long)]
        line5: Option<String>,
        #[arg(short = 'f', long)]
        line6: Option<String>,
        #[arg(short = 'g', long)]
        line7: Option<String>,
    },
    Get {
        #[arg(short, long)]
        id: String,
    },
    List,
    Update {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        kind: String,
        #[arg(short = 'a', long)]
        line1: Option<String>,
        #[arg(short = 'b', long)]
        line2: Option<String>,
        #[arg(short = 'c', long)]
        line3: Option<String>,
        #[arg(short = 'd', long)]
        line4: Option<String>,
        #[arg(short = 'e', long)]
        line5: Option<String>,
        #[arg(short = 'f', long)]
        line6: Option<String>,
        #[arg(short = 'g', long)]
        line7: Option<String>,
    },
    Delete {
        #[arg(short, long)]
        id: String,
    },
}

pub fn run(cli: Cli) {
    let repo: Box<dyn AddressRepository> = Box::new(FileBasedAddressRepository::new(
        "addresses.json".to_string(),
    ));
    let mut service = AddressService::new(repo);

    match cli.command {
        Commands::Convert { id } => convert_address(&service, &id),
        Commands::Add {
            kind,
            line1,
            line2,
            line3,
            line4,
            line5,
            line6,
            line7,
        } => add_address(
            &mut service,
            kind,
            line1,
            line2,
            line3,
            line4,
            line5,
            line6,
            line7,
        ),
        Commands::Get { id } => get_address(&service, &id),
        Commands::List => list_addresses(&service),
        Commands::Update {
            id,
            kind,
            line1,
            line2,
            line3,
            line4,
            line5,
            line6,
            line7,
        } => update_address(
            &mut service,
            id,
            kind,
            line1,
            line2,
            line3,
            line4,
            line5,
            line6,
            line7,
        ),
        Commands::Delete { id } => delete_address(&mut service, &id),
    }
}

fn convert_address(service: &AddressService, id: &str) {
    match service.get_address(id) {
        Some(iso_address) => println!("{:#?}", service.convert_to_french(&iso_address)),
        None => println!("Address with ID {} not found.", id),
    }
}

fn add_address(
    service: &mut AddressService,
    kind: String,
    line1: Option<String>,
    line2: Option<String>,
    line3: Option<String>,
    line4: Option<String>,
    line5: Option<String>,
    line6: Option<String>,
    line7: Option<String>,
) {
    let id = Uuid::new_v4().to_string();
    let kind = match kind.to_lowercase().as_str() {
        "company" => AddressKind::Company,
        "particular" => AddressKind::Particular,
        _ => {
            eprintln!("Invalid address kind. Use 'company' or 'particular'.");
            return;
        }
    };

    let french_address = FrenchAddress {
        id: id.clone(),
        line1,
        line2,
        line3,
        line4,
        line5,
        line6,
        line7,
    };
    let converted_address = service.convert_to_iso(&french_address, kind);

    if service.add_address(converted_address).is_ok() {
        println!("Address added successfully with ID: {}", id);
    } else {
        println!("Failed to add address.");
    }
}

fn get_address(service: &AddressService, id: &str) {
    match service.get_address(id) {
        Some(address) => println!("{:#?}", address),
        None => println!("Address with ID {} not found.", id),
    }
}

fn list_addresses(service: &AddressService) {
    let addresses = service.get_all_addresses();
    if addresses.is_empty() {
        println!("No addresses found.");
    } else {
        for address in addresses {
            println!("{:#?}", address);
        }
    }
}

fn update_address(
    service: &mut AddressService,
    id: String,
    kind: String,
    line1: Option<String>,
    line2: Option<String>,
    line3: Option<String>,
    line4: Option<String>,
    line5: Option<String>,
    line6: Option<String>,
    line7: Option<String>,
) {
    if let Some(existing_address) = service.get_address(&id) {
        let kind = match kind.to_lowercase().as_str() {
            "company" => AddressKind::Company,
            "particular" => AddressKind::Particular,
            _ => {
                eprintln!("Invalid address kind. Use 'company' or 'particular'.");
                return;
            }
        };

        let updated_french_address = FrenchAddress {
            id: id.clone(),
            line1: line1.or(existing_address.street_name.clone()),
            line2: line2.or(existing_address.department.clone()),
            line3: line3.or(existing_address.floor.clone()),
            line4: line4.or(existing_address.street_name.clone()),
            line5: line5.or(existing_address.post_box.clone()),
            line6: line6.or(existing_address.post_code.clone()),
            line7: line7.or(existing_address.country.clone()),
        };
        let updated_address = service.convert_to_iso(&updated_french_address, kind);

        if service.update_address(updated_address).is_ok() {
            println!("Address with ID {} updated successfully.", id);
        } else {
            println!("Failed to update address.");
        }
    } else {
        println!("Address with ID {} not found.", id);
    }
}

fn delete_address(service: &mut AddressService, id: &str) {
    if service.delete_address(id).is_ok() {
        println!("Address with ID {} deleted successfully.", id);
    } else {
        println!("Failed to delete address or address not found.");
    }
}
