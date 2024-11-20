use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use log::{error, info};

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
pub fn read_json_file<T: DeserializeOwned>(file_path: &str) -> Result<T, String> {
    if !Path::new(file_path).exists() {
        let msg = format!("File not found: {}", file_path);
        error!("{}", msg);
        return Err(msg);
    }

    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            match serde_json::from_str(&file_content) {
                Ok(parsed) => {
                    info!("Successfully parsed JSON file: {}", file_path);
                    Ok(parsed)
                }
                Err(e) => {
                    let msg = format!("Failed to parse JSON file: {}. Error: {}", file_path, e);
                    error!("{}", msg);
                    Err(msg)
                }
            }
        }
        Err(e) => {
            let msg = format!("Failed to read file: {}. Error: {}", file_path, e);
            error!("{}", msg);
            Err(msg)
        }
    }
}