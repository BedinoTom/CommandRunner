use std::process::Command;
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