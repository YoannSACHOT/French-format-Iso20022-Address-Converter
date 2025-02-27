use assert_cmd::Command;
use predicates::str::contains;
use regex::Regex;
use std::path::Path;
use std::{fs, str};

#[test]
fn test_cli_end_to_end() {
    let json_path = "addresses.json";
    let _file_manager = TestFileManager::new(json_path); // Assure la suppression après le test

    println!("🚀 Ajout d'une nouvelle adresse...");

    // Ajoute une adresse
    let mut cmd = Command::cargo_bin("fraddriso20022").unwrap();
    let add_output = cmd
        .arg("add")
        .arg("--kind")
        .arg("company")
        .arg("-a")
        .arg("DURAND SA")
        .arg("-b")
        .arg("Purchasing Department")
        .arg("-c")
        .arg("Industrial Zone")
        .arg("-d")
        .arg("22BIS Rue des Fleurs")
        .arg("-e")
        .arg("BP 40122")
        .arg("-f")
        .arg("33506 LIBOURNE CEDEX")
        .arg("-g")
        .arg("France")
        .assert()
        .success()
        .stdout(contains("Address added successfully"))
        .get_output()
        .stdout
        .clone();

    // Convertir la sortie en chaîne de caractères
    let add_stdout = str::from_utf8(&add_output).expect("Invalid UTF-8 output");

    // Extraire l'ID généré dans la sortie
    let id_regex = Regex::new(r"Address added successfully with ID: ([a-f0-9-]+)").unwrap();
    let id = id_regex
        .captures(add_stdout)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .expect("Failed to extract ID");

    println!("✅ ID extrait : {}", id);

    // Pause pour éviter une race condition avec l'écriture du fichier
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Vérifier si le fichier existe avant de récupérer l'adresse
    assert!(
        Path::new(json_path).exists(),
        "❌ Le fichier {} devrait exister après l'ajout d'une adresse !",
        json_path
    );

    println!(
        "📂 Vérification du contenu de '{}' après l'ajout...",
        json_path
    );

    // Récupérer l'adresse ajoutée en utilisant l'ID extrait
    let mut cmd_get = Command::cargo_bin("fraddriso20022").unwrap();
    cmd_get
        .arg("get")
        .arg("--id")
        .arg(&id)
        .assert()
        .success()
        .stdout(contains("DURAND SA"));

    println!("✅ Adresse récupérée avec succès !");

    // 🔄 **Mise à jour de l'adresse**
    println!("✏️ Mise à jour de l'adresse...");
    let mut cmd_update = Command::cargo_bin("fraddriso20022").unwrap();
    cmd_update
        .arg("update")
        .arg("--id")
        .arg(&id)
        .arg("--kind")
        .arg("company")
        .arg("-b")
        .arg("Updated Department")
        .arg("-c")
        .arg("New Industrial Zone")
        .assert()
        .success()
        .stdout(contains("updated successfully"));

    // Vérifier que l'adresse a bien été mise à jour
    let mut cmd_get_updated = Command::cargo_bin("fraddriso20022").unwrap();
    cmd_get_updated
        .arg("get")
        .arg("--id")
        .arg(&id)
        .assert()
        .success()
        .stdout(contains("Updated Department"))
        .stdout(contains("New Industrial Zone"));

    println!("✅ Adresse mise à jour avec succès !");

    // 🔄 **Conversion en adresse française**
    println!("🔄 Conversion en adresse française...");
    let mut cmd_convert = Command::cargo_bin("fraddriso20022").unwrap();
    cmd_convert
        .arg("convert")
        .arg("--id")
        .arg(&id)
        .assert()
        .success()
        .stdout(contains("Updated Department")) // ✅ Correspond à la mise à jour
        .stdout(contains("New Industrial Zone")) // ✅ Correspond à la mise à jour
        .stdout(contains("Rue des Fleurs"))
        .stdout(contains("BP 40122"))
        .stdout(contains("33506")) // ✅ Corrige l'espace en trop dans "33506 "
        .stdout(contains("France"));


    println!("✅ Conversion réussie !");

    // 🗑 **Suppression de l'adresse**
    println!("🗑️ Suppression de l'adresse...");
    let mut cmd_delete = Command::cargo_bin("fraddriso20022").unwrap();
    cmd_delete
        .arg("delete")
        .arg("--id")
        .arg(&id)
        .assert()
        .success()
        .stdout(contains("deleted successfully"));

    // Vérifier que l'adresse a bien été supprimée
    let mut cmd_get_deleted = Command::cargo_bin("fraddriso20022").unwrap();
    cmd_get_deleted
        .arg("get")
        .arg("--id")
        .arg(&id)
        .assert()
        .success()
        .stdout(predicates::str::is_match("Address with ID .* not found").unwrap());

    println!("✅ Adresse supprimée avec succès !");
}

/// Gestionnaire de fichier temporaire pour les tests
struct TestFileManager {
    file_path: String,
}

impl TestFileManager {
    /// Crée un gestionnaire avec un fichier donné
    fn new(file_path: &str) -> Self {
        let manager = TestFileManager {
            file_path: file_path.to_string(),
        };

        // Supprime le fichier s'il existe avant le test
        let _ = fs::remove_file(&manager.file_path);

        manager
    }
}

impl Drop for TestFileManager {
    fn drop(&mut self) {
        if Path::new(&self.file_path).exists() {
            if fs::remove_file(&self.file_path).is_err() {
                eprintln!("⚠️ Impossible de supprimer {}", self.file_path);
            } else {
                println!("🗑️ Fichier {} supprimé après les tests", self.file_path);
            }
        }
    }
}
