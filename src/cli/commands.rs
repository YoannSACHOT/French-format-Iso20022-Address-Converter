use crate::application::address_service::AddressService;
use crate::domain::models::FrenchAddress;
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
    /// Convert an ISO20022 address back to a French address
    Convert {
        /// Address ID
        #[arg(short, long)]
        id: String,
    },
    /// Add a new address to the repository
    Add {
        /// Address type: "company" or "particular"
        #[arg(short, long)]
        kind: String,

        /// Address line 1 (Recipient Name)
        #[arg(short = 'a', long)]
        line1: Option<String>,

        /// Address line 2 (Department, Service)
        #[arg(short = 'b', long)]
        line2: Option<String>,

        /// Address line 3 (Building, Floor, Entry)
        #[arg(short = 'c', long)]
        line3: Option<String>,

        /// Address line 4 (Street and number)
        #[arg(short = 'd', long)]
        line4: Option<String>,

        /// Address line 5 (PO Box, Additional Info)
        #[arg(short = 'e', long)]
        line5: Option<String>,

        /// Address line 6 (Postal Code and City)
        #[arg(short = 'f', long)]
        line6: Option<String>,

        /// Address line 7 (Country)
        #[arg(short = 'g', long)]
        line7: Option<String>,
    },

    /// Retrieve an address by ID
    Get {
        /// Address ID
        #[arg(short, long)]
        id: String,
    },

    /// List all addresses
    List,

    /// Update an existing address
    Update {
        /// Address ID
        #[arg(short, long)]
        id: String,

        /// Address type: "company" or "particular"
        #[arg(short, long)]
        kind: String,

        /// Address line 1 (Recipient Name)
        #[arg(short = 'a', long)]
        line1: Option<String>,

        /// Address line 2 (Department, Service)
        #[arg(short = 'b', long)]
        line2: Option<String>,

        /// Address line 3 (Building, Floor, Entry)
        #[arg(short = 'c', long)]
        line3: Option<String>,

        /// Address line 4 (Street and number)
        #[arg(short = 'd', long)]
        line4: Option<String>,

        /// Address line 5 (PO Box, Additional Info)
        #[arg(short = 'e', long)]
        line5: Option<String>,

        /// Address line 6 (Postal Code and City)
        #[arg(short = 'f', long)]
        line6: Option<String>,

        /// Address line 7 (Country)
        #[arg(short = 'g', long)]
        line7: Option<String>,
    },

    /// Delete an address
    Delete {
        /// Address ID
        #[arg(short, long)]
        id: String,
    },
}

/// Runs the CLI commands
pub fn run(cli: Cli) {
    let repo = FileBasedAddressRepository::new("addresses.json".to_string());
    let mut service = AddressService::new(repo);

    match cli.command {
        Commands::Convert { id } => {
            if let Some(iso_address) = service.get_address(&id) {
                let french_address = service.convert_to_french(&iso_address);
                println!("{:#?}", french_address);
            } else {
                println!("Address with ID {} not found.", id);
            }
        }
        Commands::Add {
            kind,
            line1,
            line2,
            line3,
            line4,
            line5,
            line6,
            line7,
        } => {
            let id = Uuid::new_v4().to_string();
            let kind = match kind.to_lowercase().as_str() {
                "company" => AddressKind::Company,
                "particular" => AddressKind::Particular,
                _ => {
                    eprintln!("Invalid address kind. Use 'company' or 'particular'.");
                    return;
                }
            };

            println!("DEBUG - AddressKind: {:?}", kind);

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

        Commands::Get { id } => match service.get_address(&id) {
            Some(address) => println!("{:#?}", address),
            None => println!("Address with ID {} not found.", id),
        },

        Commands::List => {
            let addresses = service.get_all_addresses();
            if addresses.is_empty() {
                println!("No addresses found.");
            } else {
                for address in addresses {
                    println!("{:#?}", address);
                }
            }
        }

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
        } => {
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

        Commands::Delete { id } => {
            if service.delete_address(&id).is_ok() {
                println!("Address with ID {} deleted successfully.", id);
            } else {
                println!("Failed to delete address or address not found.");
            }
        }
    }
}
