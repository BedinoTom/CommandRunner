// Module: tests
#[cfg(test)]
mod tests {
    use command_runner::utils::replace_templates;
    use std::collections::HashMap;

    #[test]
    fn test_replace_all_templates() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), "Alice".to_string());
        params.insert("greeting".to_string(), "Hello".to_string());

        let command = "{greeting}, {name}!";
        let result = replace_templates(command, &params);

        assert_eq!(result, Ok("Hello, Alice!".to_string()));
    }

    #[test]
    fn test_missing_single_parameter() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), "Alice".to_string());

        let command = "Hello, {name}! How's {day}?";
        let result = replace_templates(command, &params);

        assert_eq!(
            result,
            Err("Missing parameters: [\"day\"]".to_string())
        );
    }

    #[test]
    fn test_no_templates_in_command() {
        let params = HashMap::new();
        let command = "Hello, world!";
        let result = replace_templates(command, &params);

        assert_eq!(result, Ok("Hello, world!".to_string()));
    }

    #[test]
    fn test_empty_params() {
        let params = HashMap::new();
        let command = "Hello, {name}!";
        let result = replace_templates(command, &params);

        assert_eq!(
            result,
            Err("Missing parameters: [\"name\"]".to_string())
        );
    }

    #[test]
    fn test_multiple_missing_parameters() {
        let mut params = HashMap::new();
        params.insert("greeting".to_string(), "Hi".to_string());

        let command = "{greeting}, {name}! How's {day}?";
        let result = replace_templates(command, &params);

        assert_eq!(
            result,
            Err("Missing parameters: [\"name\", \"day\"]".to_string())
        );
    }

    #[test]
    fn test_repeated_templates() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), "Bob".to_string());

        let command = "Hello, {name}. Nice to see you, {name}!";
        let result = replace_templates(command, &params);

        assert_eq!(result, Ok("Hello, Bob. Nice to see you, Bob!".to_string()));
    }

    #[test]
    fn test_nested_templates_not_allowed() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), "Charlie".to_string());
        params.insert("{greeting}".to_string(), "Hi".to_string());

        let command = "{greeting}, {name}!";
        let result = replace_templates(command, &params);

        // "{greeting}" should not be replaced because nested replacement isn't supported
        assert_eq!(result, Err("Missing parameters: [\"greeting\"]".to_string()));
    }
}