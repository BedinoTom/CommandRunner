use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandTemplate {
    commands: Vec<String>, // Liste des commandes avec des templates
}

impl CommandTemplate {
    pub fn get_commands(&self) -> &Vec<String> {
        &self.commands
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    parameters: Vec<std::collections::HashMap<String, String>>, // Un tableau de maps pour chaque commande
}

impl Params {
    pub fn get_parameters(&self) -> &Vec<std::collections::HashMap<String, String>> {
        &self.parameters
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    command_config_file: String,
    parameters_config_file: String,
}

impl Config {
    pub fn get_command_config_file(&self) -> &String {
        &self.command_config_file
    }

    pub fn get_parameters_config_file(&self) -> &String {
        &self.parameters_config_file
    }
}

// Fonction pour lire un fichier JSON
pub fn read_json_file<T: for<'de> serde::Deserialize<'de>>(file_path: &str) -> T {
    let file_content = fs::read_to_string(file_path).expect(format!("Failed to read file: {}", file_path).as_str());
    serde_json::from_str(&file_content).expect(format!("Failed to parse JSON file: {}", file_path).as_str())
}