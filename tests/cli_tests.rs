use assert_cmd::Command;
use predicates::str::contains;
use regex::Regex;
use std::{fs, str};

#[test]
fn test_cli_add_and_get() {
    let json_path = "addresses.json"; // Chemin du fichier JSON

    // Assurer que le fichier est supprimé avant le test
    let _ = fs::remove_file(json_path); // Ignore les erreurs si le fichier n'existe pas

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

    println!("Extracted ID: {}", id);

    // Pause pour éviter une race condition avec l'écriture du fichier
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Récupérer l'adresse ajoutée en utilisant l'ID extrait
    let mut cmd_get = Command::cargo_bin("fraddriso20022").unwrap();
    cmd_get
        .arg("get")
        .arg("--id")
        .arg(&id)
        .assert()
        .success()
        .stdout(contains("DURAND SA"));

    // Supprime le fichier JSON après le test
    if fs::remove_file(json_path).is_err() {
        eprintln!("⚠️  Impossible de supprimer {}", json_path);
    }
}
