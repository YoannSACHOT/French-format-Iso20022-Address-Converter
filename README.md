# fraddriso20022

## Description

**fraddriso20022** est une application en ligne de commande et via une API REST conçue pour convertir les adresses postales françaises au format ISO 20022 et inversement. Construit selon les principes de la Clean Architecture, le projet garantit une forte modularité, une maintenabilité accrue et une facilité d'extension future.

## Features

- **Conversion d'adresses** :
    - Convertir des adresses postales françaises en format ISO 20022.
    - Convertir des adresses ISO 20022 en format français.
- **Opérations CRUD** :
    - Ajouter, récupérer, mettre à jour et supprimer des adresses stockées dans un dépôt local (JSON), MongoDB ou en mémoire.
- **Patterns de dépôt** :
    - Supporte plusieurs backends de stockage :
        - Fichier (JSON)
        - En mémoire (pour les tests)
        - MongoDB
        - PostgreSQL (*prévu*)
- **Clean Architecture** :
    - Séparation claire de la logique métier (domaine) et des préoccupations d'infrastructure.
- **Interface en ligne de commande (CLI)** :
    - Interface intuitive basée sur [clap](https://crates.io/crates/clap).
- **API REST** :
    - Expose les mêmes fonctionnalités via des endpoints REST, construits avec [Actix-web](https://actix.rs/).

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (version stable recommandée)

## Installation

1. **Cloner le dépôt**  
   ```sh
   git clone https://github.com/yoannsachot/fraddriso20022.git
   cd fraddriso20022
   ```

2. **Construire le projet**
   ```sh
   cargo build --release
   ```

## Variables d'environnement

Pour configurer le backend du dépôt, définissez les variables d'environnement suivantes :

- **SELECT_REPO** : Type de dépôt. Valeurs acceptées :
    - `file` (par défaut) : Utilise un fichier JSON (`addresses.json`).
    - `inmemory` : Utilise un dépôt en mémoire (pour les tests ou usage éphémère).
    - `mongo` ou `mongodb` : Utilise MongoDB. Dans ce cas :
        - **MONGO_URI** : Chaîne de connexion MongoDB (exemple : `mongodb://localhost:27017`).
        - **MONGO_DB_NAME** : Nom de la base de données (par défaut `addresses_db`).
        - **MONGO_DB_COLLECTION** : Nom de la collection (par défaut `addresses`).
    - `postgres` : *Prévu* (non implémenté actuellement).

Exemple (shell UNIX) :
```sh
export SELECT_REPO=mongo
export MONGO_URI="mongodb://localhost:27017"
export MONGO_DB_NAME="addresses_db"
export MONGO_DB_COLLECTION="addresses"
```

Exemple (PowerShell sous Windows) :
```powershell
$env:SELECT_REPO="mongo"
$env:MONGO_URI="mongodb://localhost:27017"
$env:MONGO_DB_NAME="addresses_db"
$env:MONGO_DB_COLLECTION="addresses"
```

## Usage

### Interface en ligne de commande (CLI)

Voici quelques commandes courantes après construction :

#### Ajouter une adresse
```sh
cargo run --bin fraddriso20022-cli -- add --kind <company|particular> \
    -a "Nom du destinataire/société" \
    -b "Département ou numéro de chambre" \
    -c "Informations d'étage ou d'entrée" \
    -d "Numéro et nom de la rue" \
    -e "BP ou informations complémentaires" \
    -f "Code postal et ville" \
    -g "Pays"
```
- **--kind** : Type d'adresse (`company` ou `particular`).
- **-a** à **-g** : Champs facultatifs pour renseigner les lignes d'adresse.

#### Lister toutes les adresses
```sh
cargo run --bin fraddriso20022-cli -- list
```

#### Récupérer une adresse par ID
```sh
cargo run --bin fraddriso20022-cli -- get --id <ID>
```

#### Convertir une adresse (ISO 20022 → Français)
```sh
cargo run --bin fraddriso20022-cli -- convert --id <ID>
```
Affiche l'adresse au format français.

#### Mettre à jour une adresse
```sh
cargo run --bin fraddriso20022-cli -- update --id <ID> --kind <company|particular> \
    -a "Nouveau nom" \
    -b "Nouveau département/chambre" \
    ...
```
Met à jour uniquement les champs renseignés.

#### Supprimer une adresse
```sh
cargo run --bin fraddriso20022-cli -- delete --id <ID>
```

### API REST

Le projet propose également une API REST permettant d'interagir avec les adresses.

#### Démarrer le serveur REST

Pour lancer le serveur REST, exécutez :
```sh
cargo run --bin fraddriso20022-rest
```
Le serveur démarre sur [http://127.0.0.1:8080](http://127.0.0.1:8080).

#### Endpoints disponibles

- **GET /addresses**  
  Récupère la liste de toutes les adresses.

- **GET /addresses/{id}**  
  Récupère une adresse spécifique par son ID.

- **POST /addresses**  
  Ajoute une nouvelle adresse.  
  **Corps de la requête (JSON) :**
  ```json
  {
      "kind": "company|particular",
      "line1": "Nom du destinataire/société",
      "line2": "Département ou chambre",
      "line3": "Étage ou informations d'entrée",
      "line4": "Rue et numéro",
      "line5": "BP ou information complémentaire",
      "line6": "Code postal et ville",
      "line7": "Pays"
  }
  ```

- **PUT /addresses/{id}**  
  Met à jour une adresse existante par son ID.  
  **Corps de la requête (JSON) :** (Les champs omis conservent leurs valeurs existantes)
  ```json
  {
      "kind": "company|particular",
      "line1": "Nouveau nom",
      "line2": "Nouveau département ou chambre",
      "line3": "Nouvel étage ou infos d'entrée",
      "line4": "Nouvelle rue et numéro",
      "line5": "Nouveau BP ou infos complémentaires",
      "line6": "Nouveau code postal et ville",
      "line7": "Nouveau pays"
  }
  ```

- **DELETE /addresses/{id}**  
  Supprime une adresse par son ID.

- **GET /addresses/{id}/convert**  
  Convertit une adresse ISO 20022 stockée en son équivalent au format français.

#### Tester l'API REST

Vous pouvez utiliser des outils comme [Postman](https://www.postman.com/) ou `curl` pour tester les endpoints, par exemple :
```sh
curl http://127.0.0.1:8080/addresses
```

## Structure du projet

```
fraddriso20022/
├── Cargo.toml
├── LICENSE.md
├── README.md
├── src/
│   ├── main.rs                 # Point d'entrée CLI
│   ├── rest_main.rs            # Point d'entrée API REST
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
│       ├── app_state.rs
│       ├── file_repository.rs
│       ├── in_memory_repository.rs
│       ├── mongo_repository.rs
│       └── rest_controller.rs
├── tests/
│   ├── cli_tests.rs
│   └── integration_tests.rs
└── .github/
    └── workflows/
        └── build-test.yml
```

## Tests

Exécutez tous les tests unitaires et d'intégration avec :
```sh
cargo test --verbose
```

## Architecture & Extensibilité

### Clean Architecture

- **Couche Domaine**  
  Contient les modèles internes (format français et ISO 20022) et la logique métier.

- **Couche Application**  
  Implémente les cas d'utilisation via `AddressService`, en masquant les détails d'implémentation du dépôt et de la conversion.

- **Couche Infrastructure**  
  Fournit les implémentations concrètes du pattern de dépôt :
    - Fichier JSON
    - Mémoire (pour les tests)
    - MongoDB
    - PostgreSQL (prévu)
    - Intègre également le controller REST.

- **Couche CLI**  
  Gère les interactions utilisateur via des commandes en ligne de commande.

### Extensibilité future

- **Nouvelles interfaces** :  
  Ajoutez facilement une interface graphique ou une autre API sans modifier la logique métier.
- **Nouveaux dépôts** :  
  Intégrez d'autres solutions de stockage en implémentant le trait `AddressRepository`.

## Intégration Continue

GitHub Actions est utilisé pour automatiser les builds et les tests à chaque push ou pull request.  
Voir [`.github/workflows/build-test.yml`](.github/workflows/build-test.yml) pour plus de détails.

## Contribution

Les contributions sont les bienvenues ! Pour contribuer :

1. **Forkez** le dépôt.
2. **Créez une branche de fonctionnalité** (ex. : `feature/new-functionality`).
3. **Commitez** vos changements avec des messages clairs.
4. **Soumettez une pull request** pour révision.

## License

Ce projet est sous licence MIT. Voir [LICENSE.md](LICENSE.md) pour les détails.