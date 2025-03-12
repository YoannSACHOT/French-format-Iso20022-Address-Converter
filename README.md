# fraddriso20022

## Description

**fraddriso20022** is a command-line application designed to convert French postal addresses to ISO 20022 format and vice versa. Built with clean architecture principles, the project ensures high modularity, maintainability, and ease of future extension.

## Features

- **Address Conversion**:
    - Convert French postal addresses to ISO 20022 format.
    - Convert ISO 20022 addresses back to French format.
- **CRUD Operations**:
    - Add, retrieve, update, and delete addresses stored in a local JSON repository, MongoDB, or in-memory store.
- **Repository Patterns**:
    - Supports multiple storage backends:
        - File-based (JSON)
        - In-memory (for testing)
        - MongoDB
        - PostgreSQL (planned)
- **Clean Architecture**:
    - Clear separation of business logic (domain) from infrastructure concerns.
- **Command-Line Interface**:
    - Intuitive CLI built with [clap](https://crates.io/crates/clap) to interact with the application.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

## Installation

1. **Clone the Repository**  
   ```sh
   git clone https://github.com/yoannsachot/fraddriso20022.git
   cd fraddriso20022
   ```

2. **Build the Project**
   ```sh
   cargo build --release
   ```

3. **Run the Application**
   ```sh
   cargo run -- --help
   ```

## Environment Variables

To configure the repository backend, set the following environment variables:

- `SELECT_REPO`: Defines the repository type. Accepted values:
    - `file` (default): Uses a JSON file (`addresses.json`) for storage.
    - `inmemory`: Uses an in-memory repository (for testing or ephemeral use).
    - `mongo` or `mongodb`: Uses MongoDB for storage. In this case:
        - `MONGO_URI`: Required. The MongoDB connection string (e.g., `mongodb://localhost:27017`).
        - `MONGO_DB_NAME`: Name of the MongoDB database (defaults to `addresses_db` if unset).
        - `MONGO_DB_COLLECTION`: Collection name (defaults to `addresses` if unset).
    - `postgres`: *Planned* (currently not implemented in the code).

Example (MongoDB on UNIX shell):
```sh
export SELECT_REPO=mongo
export MONGO_URI="mongodb://localhost:27017"
export MONGO_DB_NAME="addresses_db"
export MONGO_DB_COLLECTION="addresses"
```

On Windows (PowerShell):
```powershell
$env:SELECT_REPO="mongo"
$env:MONGO_URI="mongodb://localhost:27017"
$env:MONGO_DB_NAME="addresses_db"
$env:MONGO_DB_COLLECTION="addresses"
```

## Usage

Below are some common commands you can run after building the project:

### Add an Address
```sh
cargo run -- add --kind <company|particular> \
    -a "Recipient/Company Name" \
    -b "Department/Sub-Department or Room" \
    -c "Floor info" \
    -d "Street number and name" \
    -e "PO Box" \
    -f "Postal Code and City" \
    -g "Country"
```
- **--kind**: Address type (`company` or `particular`).
- **-a**: Recipient name (optional).
- **-b**: Department or room (depending on `company` or `particular`).
- **-c**: Floor or entry details (optional).
- **-d**: Street and number (optional).
- **-e**: PO Box or additional info (optional).
- **-f**: Postal code and city (optional).
- **-g**: Country (optional).

### List All Addresses
```sh
cargo run -- list
```

### Retrieve an Address by ID
```sh
cargo run -- get --id <ID>
```

### Convert an Address (ISO 20022 → French)
```sh
cargo run -- convert --id <ID>
```
Displays the address in French format.

### Update an Address
```sh
cargo run -- update --id <ID> --kind <company|particular> \
    -a "New Name" \
    -b "New Dept or Room" \
    ...
```
Updates only the fields you provide; unspecified fields are left unchanged.

### Delete an Address
```sh
cargo run -- delete --id <ID>
```

## Project Structure

```
fraddriso20022/
├── Cargo.toml
├── LICENSE.md
├── README.md
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
│   │   └── validation.rs
│   └── infrastructure/
│       ├── file_repository.rs
│       ├── in_memory_repository.rs
│       ├── mongo_repository.rs
│       └── mod.rs
├── tests/
│   ├── cli_tests.rs
│   └── integration_tests.rs
└── .github/
    └── workflows/
        └── build-test.yml
```

## Testing

Run all unit and integration tests with:
```sh
cargo test --verbose
```

## Architecture & Extensibility

### Clean Architecture

- **Domain Layer**  
  Contains the internal models (for both French and ISO 20022 formats) and business logic.

- **Application Layer**  
  Implements use cases through the `AddressService`, abstracting repository operations and conversion logic.

- **Infrastructure Layer**  
  Provides concrete implementations of the repository pattern:
    - File-based repository (using JSON)
    - In-memory repository (ideal for testing)
    - MongoDB repository
    - PostgreSQL repository (planned)

- **CLI Layer**  
  Manages user interactions via command-line commands, translating inputs into service calls.

### Future Extensibility

- **New Interfaces**:  
  Easily add a REST API or GUI without altering core business logic.
- **Additional Repositories**:  
  Integrate with other storage solutions by implementing the `AddressRepository` trait.

## Continuous Integration

GitHub Actions is used to automate builds and tests on every push and pull request.  
See [`.github/workflows/build-test.yml`](.github/workflows/build-test.yml) for details.

## Contribution

Contributions are welcome! To contribute:

1. **Fork** the repository.
2. **Create a feature branch** (e.g., `feature/new-functionality`).
3. **Commit** your changes with clear messages.
4. **Submit a pull request** for review.

## License

This project is licensed under the MIT License. See [LICENSE.md](LICENSE.md) for details.