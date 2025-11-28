use regex::Regex;

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub takes_value: bool,
}

pub fn parse_help_output(help_text: &str) -> Vec<Argument> {
    let mut arguments = Vec::new();

    let option_regex = Regex::new(r"(?m)^\s*(-[a-zA-Z]|--[\w-]+)(?:[\s,]+(-[a-zA-Z]|--[\w-]+))?(?:\s+<([^>]+)>|\s+\[([^\]]+)\])?\s+(.+?)$").unwrap();

    for cap in option_regex.captures_iter(help_text) {
        let long_flag = cap.get(2)
            .or_else(|| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        if long_flag.is_empty() {
            continue;
        }

        let takes_value = cap.get(3).is_some() || cap.get(4).is_some();
        let description = cap.get(5)
            .map(|m| m.as_str().trim())
            .unwrap_or("")
            .to_string();

        let required = cap.get(3).is_some();

        arguments.push(Argument {
            name: long_flag.to_string(),
            description,
            required,
            takes_value,
        });
    }

    arguments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_options() {
        let help = r#"
Usage: mycommand [OPTIONS]

Options:
  -h, --help       Print help
  -v, --verbose    Enable verbose output
  --output <FILE>  Output file path
  --count [NUM]    Number of items
        "#;

        let args = parse_help_output(help);
        assert!(args.len() >= 2);

        let verbose = args.iter().find(|a| a.name == "--verbose");
        assert!(verbose.is_some());
        assert!(!verbose.unwrap().takes_value);
    }

    #[test]
    fn test_parse_with_values() {
        let help = r#"
Options:
  --input <FILE>   Input file (required)
  --config [CFG]   Config file (optional)
        "#;

        let args = parse_help_output(help);

        let input = args.iter().find(|a| a.name == "--input");
        assert!(input.is_some());
        assert!(input.unwrap().takes_value);
        assert!(input.unwrap().required);

        let config = args.iter().find(|a| a.name == "--config");
        assert!(config.is_some());
        assert!(config.unwrap().takes_value);
        assert!(!config.unwrap().required);
    }
}
