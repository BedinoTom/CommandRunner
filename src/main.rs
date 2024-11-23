mod config;
mod utils;

use std::env;
use log::{error, info, LevelFilter};
use config::{CommandTemplate, Config, Params, read_json_file};
use utils::{execute_command, replace_templates};

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let file_path = env::var("CONFIG_FILE").unwrap_or_else(|_e| "config.json".to_string());

    let config: Config = match read_json_file::<Config>(&file_path) {
        Ok(config) => {
            info!("Configuration loaded successfully: {:?}", config);
            config
        }
        Err(err) => {
            error!("Critical error: failed to load configuration: {}", err);
            panic!("Failed to load configuration. Exiting."); // Arrête le programme proprement
        }
    };

    // Lire les fichiers JSON
    let command_templates: CommandTemplate = match read_json_file::<CommandTemplate>(config.get_command_config_file().as_str()) {
        Ok(command_templates) => {
            info!("Command templates loaded successfully: {:?}", command_templates);
            command_templates
        }
        Err(err) => {
            error!("Critical error: failed to commands file: {}", err);
            panic!("Failed to load configuration. Exiting."); // Arrête le programme proprement
        }
    };
    let params: Params = match read_json_file::<Params>(config.get_parameters_config_file().as_str()) {
        Ok(params) => {
            info!("Parameters loaded successfully: {:?}", params);
            params
        }
        Err(err) => {
            error!("Critical error: failed to params file: {}", err);
            panic!("Failed to load configuration. Exiting."); // Arrête le programme proprement
        }
    };

    // Vérification que les tailles des deux tableaux sont identiques
    if command_templates.get_commands().len() != params.get_parameters().len() {
        eprintln!("Error: The number of commands and the number of parameter sets do not match.");
        return;
    }

    // Remplacer les templates dans les commandes et les exécuter
    for (i, command) in command_templates.get_commands().iter().enumerate() {
        let param_set = &params.get_parameters()[i]; // Récupérer le jeu de paramètres correspondant
        match replace_templates(command, param_set) {
            Ok(final_command) => {
                println!("Executing: {}", final_command);
                execute_command(&final_command);
            }
            Err(e) => {
                eprintln!("Error in command {}: {}", i + 1, e);
                panic!("Failed to execute command. Exiting."); // Arrête le programme proprement
            }
        }
    }
}
