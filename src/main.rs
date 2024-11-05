use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct CommandTemplate {
    commands: Vec<String>, // Liste des commandes avec des templates
}

#[derive(Serialize, Deserialize, Debug)]
struct Params {
    parameters: Vec<std::collections::HashMap<String, String>>, // Un tableau de maps pour chaque commande
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    command_config_file: String,
    parameters_config_file: String,
}

// Fonction pour lire un fichier JSON
fn read_json_file<T: for<'de> serde::Deserialize<'de>>(file_path: &str) -> T {
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    serde_json::from_str(&file_content).expect("Failed to parse JSON")
}

// Fonction pour remplacer les templates dans une commande
fn replace_templates(command: &str, params: &std::collections::HashMap<String, String>) -> Result<String, String> {
    let re = Regex::new(r"\{(\w+)\}").unwrap(); // Regex pour capturer {X}

    let mut missing_params = Vec::new();
    let result = re.replace_all(command, |caps: &regex::Captures| {
        let var_name = &caps[1];
        match params.get(var_name) {
            Some(value) => value.clone(),
            None => {
                missing_params.push(var_name.to_string());
                format!("{{{}}}", var_name)
            }
        }
    }).to_string();

    if missing_params.is_empty() {
        Ok(result)
    } else {
        Err(format!("Missing parameters: {:?}", missing_params))
    }
}

// Fonction pour exécuter une commande
fn execute_command(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect(); // Sépare la commande en parties
    let status = Command::new(parts[0])
        .args(&parts[1..])
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("Command '{}' executed successfully", command);
    } else {
        println!("Command '{}' failed to execute", command);
    }
}

fn main() {
    let optional_value = option_env!("CONFIG_FILE");
    let file_path = optional_value
        .unwrap_or("./config.json")
        .to_string().to_owned();

    let config: Config = read_json_file(&file_path);

    // Lire les fichiers JSON
    let command_templates: CommandTemplate = read_json_file(config.command_config_file.as_str());
    let params: Params = read_json_file(config.parameters_config_file.as_str());

    // Vérification que les tailles des deux tableaux sont identiques
    if command_templates.commands.len() != params.parameters.len() {
        eprintln!("Error: The number of commands and the number of parameter sets do not match.");
        return;
    }

    // Remplacer les templates dans les commandes et les exécuter
    for (i, command) in command_templates.commands.iter().enumerate() {
        let param_set = &params.parameters[i]; // Récupérer le jeu de paramètres correspondant
        match replace_templates(command, param_set) {
            Ok(final_command) => {
                println!("Executing: {}", final_command);
                execute_command(&final_command);
            }
            Err(e) => {
                eprintln!("Error in command {}: {}", i + 1, e);
            }
        }
    }
}
