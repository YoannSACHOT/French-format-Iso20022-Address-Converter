# Étape 1 : Construction du binaire avec l'image officielle de Rust
FROM rust:latest as builder
WORKDIR /app
# Copier le fichier de configuration et le code source
COPY Cargo.toml Cargo.lock ./
COPY . .
# Compiler le binaire du serveur REST en mode release
RUN cargo build --release --bin fraddriso20022-rest

# Étape 2 : Créer une image d'exécution basée sur Ubuntu 22.04
FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
# Copier le binaire depuis l'étape builder
COPY --from=builder /app/target/release/fraddriso20022-rest /usr/local/bin/
# Exposer le port du serveur REST
EXPOSE 8080
# Commande de démarrage
CMD ["/usr/local/bin/fraddriso20022-rest"]
