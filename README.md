# fraddriso20022

## Description
**fraddriso20022** is a command-line application that converts French postal addresses to ISO 20022 format and vice versa. The project follows clean architecture principles, ensuring modularity, maintainability, and extensibility.

## Features
- **Convert French addresses to ISO 20022 format**
- **Convert ISO 20022 addresses to French format**
- **Save, update, and delete addresses in a local JSON repository**
- **Retrieve addresses by ID**
- **List all stored addresses**
- **Designed with clean architecture principles**

## Installation
### Prerequisites
- Rust (latest stable version recommended)

### Clone the repository
```sh
git clone https://github.com/yoannsachot/fraddriso20022.git
cd fraddriso20022
```

### Build the project
```sh
cargo build --release
```

### Run the application
```sh
cargo run -- --help
```

## Command-Line Arguments
### Add an address
```sh
cargo run -- add --kind <company|particular> -a "Recipient Name" -b "Department" -c "Floor" -d "Street" -e "PO Box" -f "Postal Code City" -g "Country"
```
- `--kind`: Type of address (`company` or `particular`)
- `-a`: Recipient name (optional)
- `-b`: Department or service (optional)
- `-c`: Building, floor, or entry details (optional)
- `-d`: Street and number (optional)
- `-e`: PO Box or additional info (optional)
- `-f`: Postal code and city (optional)
- `-g`: Country (optional)

### List all addresses
```sh
cargo run -- list
```

### Retrieve an address by ID
```sh
cargo run -- get --id <ID>
```

### Convert an ISO 20022 address to a French address
```sh
cargo run -- convert --id <ID>
```

### Update an address
```sh
cargo run -- update --id <ID> --kind <company|particular> -a "Updated Name" -b "Updated Department"
```

### Delete an address
```sh
cargo run -- delete --id <ID>
```

## Project Structure
```
fraddriso20022/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── application/
│   │   ├── address_service.rs
│   │   └── mod.rs
│   ├── cli/
│   │   ├── commands.rs
│   │   └── mod.rs
│   ├── domain/
│   │   ├── models.rs
│   │   ├── repository.rs
│   │   ├── usecases.rs
│   │   └── mod.rs
│   ├── infrastructure/
│   │   ├── file_repository.rs
│   │   └── mod.rs
├── tests/
│   ├── cli_tests.rs
│   ├── integration_tests.rs
│   ├── infrastructure/
│   │   ├── in_memory_repository.rs
│   │   └── mod.rs
└── .github/workflows/build-test.yml
```

## Testing
Run unit and integration tests:
```sh
cargo test --verbose
```

## Clean Architecture and Extensibility
- **Internal Model**: The domain layer contains an internal model supporting both French and ISO 20022 address formats.
- **Repository Pattern**: The application supports both an in-memory repository (for unit tests) and a file-based repository (for persistence and integration tests).
- **Future Extensibility**: The architecture allows for adding new presenters (e.g., REST API) or repositories (e.g., database) with minimal changes to the domain.

## Continuous Integration
A GitHub Actions workflow (`.github/workflows/build-test.yml`) is set up to:
- Build the project
- Run tests automatically on push and pull requests

## Contribution
Contributions are welcome! To contribute:
1. **Fork** the repository
2. **Create a feature branch** (`feature/new-functionality`)
3. **Submit a pull request**

## License
This project is licensed under the MIT License.

