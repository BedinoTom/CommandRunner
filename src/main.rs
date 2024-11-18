mod config;
mod utils;

use config::{CommandTemplate, Config, Params, read_json_file};
use utils::{execute_command, replace_templates};

fn main() {
    let optional_value = option_env!("CONFIG_FILE");
    let file_path = optional_value
        .unwrap_or("./config.json")
        .to_string().to_owned();

    let config: Config = read_json_file(&file_path);

    // Lire les fichiers JSON
    let command_templates: CommandTemplate = read_json_file(config.get_command_config_file().as_str());
    let params: Params = read_json_file(config.get_parameters_config_file().as_str());

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
            }
        }
    }
}
