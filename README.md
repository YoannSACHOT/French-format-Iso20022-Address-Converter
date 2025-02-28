# fraddriso20022

## Description

**fraddriso20022** is a command-line application designed to convert French postal addresses to ISO 20022 format and vice versa. Built with clean architecture principles, the project ensures high modularity, maintainability, and ease of future extension.

## Features

- **Address Conversion**:
    - Convert French postal addresses to ISO 20022 format.
    - Convert ISO 20022 addresses back to French format.
- **CRUD Operations**:
    - Add, retrieve, update, and delete addresses stored in a local JSON repository.
- **Repository Patterns**:
    - Supports multiple storage backends:
        - File-based (JSON)
        - In-memory (for testing)
        - PostgreSQL (planned implementation)
- **Clean Architecture**:
    - Clear separation of business logic (domain) from infrastructure concerns.
- **Command-Line Interface**:
    - Intuitive CLI built with [clap](https://crates.io/crates/clap) to interact with the application.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

## Installation

### Environment Variables
Before running the application, you can set the following environment variables to configure the repository backend:

- `SELECT_REPO`: Defines the repository type. Accepted values:
    - `file` (default): Uses a JSON file for storage.
    - `inmemory`: Uses an in-memory repository (for testing purposes).
    - `postgres`: Uses a PostgreSQL database.
- `DATABASE_URL`: Required if `SELECT_REPO=postgres`, specifies the PostgreSQL connection string.

To set environment variables in a UNIX shell:
```sh
export SELECT_REPO=postgres
export DATABASE_URL="postgres://user:password@localhost/dbname"
```

On Windows (PowerShell):
```powershell
$env:SELECT_REPO="postgres"
$env:DATABASE_URL="postgres://user:password@localhost/dbname"
```

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

## Usage

### Add an Address
```sh
cargo run -- add --kind <company|particular> -a "Recipient Name" -b "Department" -c "Floor" -d "Street" -e "PO Box" -f "Postal Code City" -g "Country"
```
- **--kind**: Address type (`company` or `particular`)
- **-a**: Recipient name (optional)
- **-b**: Department or service (optional)
- **-c**: Floor or entry details (optional)
- **-d**: Street and number (optional)
- **-e**: PO Box or additional info (optional)
- **-f**: Postal code and city (optional)
- **-g**: Country (optional)

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

### Update an Address
```sh
cargo run -- update --id <ID> --kind <company|particular> -a "Updated Name" -b "Updated Department" ...
```

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
│   │   └── mod.rs
│   └── infrastructure/
│       ├── file_repository.rs
│       ├── in_memory_repository.rs
│       ├── mod.rs
│       └── postgresql_repository.rs
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

- **Domain Layer**:  
  Contains the internal models (for both French and ISO 20022 formats) and business logic.

- **Application Layer**:  
  Implements use cases through the `AddressService`, abstracting repository operations and conversion logic.

- **Infrastructure Layer**:  
  Provides concrete implementations of the repository pattern:
    - File-based repository (using JSON)
    - In-memory repository (ideal for testing)
    - PostgreSQL repository (planned)

- **CLI Layer**:  
  Manages user interactions via command-line commands, translating inputs into service calls.

### Future Extensibility

- **New Presenters**:  
  Easily add a REST API or GUI without altering core business logic.
- **Additional Repositories**:  
  Integrate with other storage solutions (e.g., cloud storage, alternative databases) by implementing the `AddressRepository` trait.

## Continuous Integration

The project uses GitHub Actions to automate builds and tests on every push and pull request. See the workflow configuration in [`.github/workflows/build-test.yml`](.github/workflows/build-test.yml).

## Contribution

Contributions are welcome! To contribute:

1. **Fork** the repository.
2. **Create a feature branch** (e.g., `feature/new-functionality`).
3. **Commit** your changes with clear messages.
4. **Submit a pull request** for review.

## License

This project is licensed under the MIT License. See [LICENSE.md](LICENSE.md) for details.