use std::process::{Command, Stdio};
use regex::Regex;

// Fonction pour remplacer les templates dans une commande
pub fn replace_templates(command: &str, params: &std::collections::HashMap<String, String>) -> Result<String, String> {
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
pub fn execute_command(command: &str) {
    // Sépare les commandes par les pipes
    let commands: Vec<&str> = command.split('|').map(|s| s.trim()).collect();

    // Stocke la sortie de la commande précédente
    let mut previous_output = None;

    for cmd in commands {
        // Sépare la commande et ses arguments
        let mut parts = cmd.split_whitespace();
        let program = parts.next().expect("Commande invalide !");
        let args: Vec<&str> = parts.collect();

        let mut command_process = Command::new(program)
            .args(args)
            .stdin(previous_output.take().map_or(Stdio::inherit(), Stdio::from)) // Utilise l'entrée standard précédente
            .spawn()
            .expect("Échec de l'exécution de la commande");

        // Attendre la fin de chaque commande
        let status = command_process.wait().expect("Échec de l'attente de la commande");
        if !status.success() {
            panic!("La commande '{}' a échoué.", cmd);
        }
    }

    println!("Toutes les commandes ont été exécutées avec succès !");
}
