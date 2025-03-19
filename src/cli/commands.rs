use clap::{Parser, Subcommand};

use crate::application::command::address_command_service::AddressCommandService;
use crate::application::query::address_query_service::AddressQueryService;
use crate::domain::models::{AddressKind, FrenchAddressBuilder, ISO20022Address};
use crate::domain::usecases::{convert_to_french, convert_to_iso};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "postal_converter_cqrs", version = "1.0")]
pub struct CliCqrs {
    #[command(subcommand)]
    pub command: CommandsCqrs,
}

#[derive(Subcommand)]
pub enum CommandsCqrs {
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
    Get {
        #[arg(short, long)]
        id: String,
    },
    List,
    Convert {
        #[arg(short, long)]
        id: String,
    },
}

pub fn run_cqrs(
    cli: CliCqrs,
    command_service: &mut AddressCommandService,
    query_service: &AddressQueryService,
) {
    match cli.command {
        CommandsCqrs::Add {
            kind,
            line1,
            line2,
            line3,
            line4,
            line5,
            line6,
            line7,
        } => add_address(
            command_service,
            kind,
            line1,
            line2,
            line3,
            line4,
            line5,
            line6,
            line7,
        ),

        CommandsCqrs::Update {
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
            command_service,
            query_service,
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

        CommandsCqrs::Delete { id } => delete_address(command_service, id),

        CommandsCqrs::Get { id } => get_address(query_service, id),
        CommandsCqrs::List => list_addresses(query_service),
        CommandsCqrs::Convert { id } => convert_address(query_service, id),
    }
}

fn add_address(
    cmd_svc: &mut AddressCommandService,
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
    let kind_enum = match parse_kind(&kind) {
        Ok(k) => k,
        Err(_) => return,
    };
    let french = match FrenchAddressBuilder::new()
        .id(id.clone())
        .line1(line1)
        .line2(line2)
        .line3(line3)
        .line4(line4)
        .line5(line5)
        .line6(line6)
        .line7(line7)
        .build()
    {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Error building FrenchAddress: {e}");
            return;
        }
    };
    let iso: ISO20022Address = convert_to_iso(&french, kind_enum);
    match cmd_svc.add_address(iso) {
        Ok(_) => println!("Address added successfully with ID: {}", id),
        Err(e) => eprintln!("Failed to add address: {e}"),
    }
}

fn update_address(
    cmd_svc: &mut AddressCommandService,
    query_svc: &AddressQueryService,
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
    let existing_iso = match query_svc.get_address(&id) {
        Some(iso) => iso,
        None => {
            eprintln!("Address with ID {id} not found.");
            return;
        }
    };

    let kind_enum = match parse_kind(&kind) {
        Ok(k) => k,
        Err(_) => return,
    };

    let existing_french = convert_to_french(&existing_iso);

    let updated_french = match FrenchAddressBuilder::new()
        .id(id.clone())
        .line1(line1.or(existing_french.line1))
        .line2(line2.or(existing_french.line2))
        .line3(line3.or(existing_french.line3))
        .line4(line4.or(existing_french.line4))
        .line5(line5.or(existing_french.line5))
        .line6(line6.or(existing_french.line6))
        .line7(line7.or(existing_french.line7))
        .build()
    {
        Ok(fa) => fa,
        Err(e) => {
            eprintln!("Error building updated address: {e}");
            return;
        }
    };

    let updated_iso = convert_to_iso(&updated_french, kind_enum);

    match cmd_svc.update_address(updated_iso) {
        Ok(_) => println!("Address with ID {} updated successfully.", id),
        Err(e) => eprintln!("Failed to update address: {e}"),
    }
}

fn delete_address(cmd_svc: &mut AddressCommandService, id: String) {
    match cmd_svc.delete_address(&id) {
        Ok(_) => println!("Address with ID {} deleted successfully.", id),
        Err(e) => eprintln!("Failed to delete address: {e}"),
    }
}

fn get_address(query_svc: &AddressQueryService, id: String) {
    match query_svc.get_address(&id) {
        Some(iso) => {
            println!("{iso:#?}");
        }
        None => eprintln!("Address with ID {id} not found."),
    }
}

fn list_addresses(query_svc: &AddressQueryService) {
    let all = query_svc.get_all_addresses();
    if all.is_empty() {
        println!("No addresses found.");
    } else {
        for iso in all {
            println!("{iso:#?}");
        }
    }
}

fn convert_address(query_svc: &AddressQueryService, id: String) {
    match query_svc.get_address(&id) {
        Some(iso) => {
            let french = convert_to_french(&iso);
            println!("{french:#?}");
        }
        None => eprintln!("Address with ID {id} not found."),
    }
}

fn parse_kind(s: &str) -> Result<AddressKind, ()> {
    match s.to_lowercase().as_str() {
        "company" => Ok(AddressKind::Company),
        "particular" => Ok(AddressKind::Particular),
        _ => {
            eprintln!("Invalid address kind. Use 'company' or 'particular'.");
            Err(())
        }
    }
}
