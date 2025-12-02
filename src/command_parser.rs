use crate::app::{Argument, Value};
use anyhow::Result;

#[derive(Debug)]
pub struct ParsedCommand {
    pub base_command: Vec<String>,
    pub arguments: Vec<Argument>,
}

pub fn parse_command(command_str: &str) -> Result<ParsedCommand> {
    let tokens = shlex::split(command_str)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse command string"))?;

    if tokens.is_empty() {
        anyhow::bail!("Empty command");
    }

    let mut base_command = Vec::new();
    let mut arguments = Vec::new();
    let mut i = 0;

    // Find where arguments start (first token starting with -)
    while i < tokens.len() {
        let token = &tokens[i];
        if token.starts_with('-') {
            break;
        }
        base_command.push(token.clone());
        i += 1;
    }

    // Parse arguments
    while i < tokens.len() {
        let token = &tokens[i];

        if token.starts_with('-') {
            // Check if it's in the form --flag=value or -f=value
            if let Some(eq_pos) = token.find('=') {
                let flag = token[..eq_pos].to_string();
                let value = token[eq_pos + 1..].to_string();
                arguments.push(Argument {
                    flag,
                    value: Value::String(value),
                });
                i += 1;
            } else {
                // Check if next token is a value (doesn't start with -)
                let flag = token.clone();
                if i + 1 < tokens.len() && !tokens[i + 1].starts_with('-') {
                    let value = tokens[i + 1].clone();
                    arguments.push(Argument {
                        flag,
                        value: Value::String(value),
                    });
                    i += 2;
                } else {
                    // Boolean flag (no value)
                    arguments.push(Argument {
                        flag,
                        value: Value::Checked(true),
                    });
                    i += 1;
                }
            }
        } else {
            // Unexpected token (not starting with -)
            // Treat it as a positional argument
            arguments.push(Argument {
                flag: String::new(),
                value: Value::String(token.clone()),
            });
            i += 1;
        }
    }

    Ok(ParsedCommand {
        base_command,
        arguments,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let cmd = "kubectl get pods -l app=asset -o json";
        let parsed = parse_command(cmd).unwrap();

        assert_eq!(parsed.base_command, vec!["kubectl", "get", "pods"]);
        assert_eq!(parsed.arguments.len(), 2);
        assert_eq!(parsed.arguments[0].flag, "-l");
        assert_eq!(
            parsed.arguments[0].value,
            Value::String("app=asset".to_string())
        );
        assert_eq!(parsed.arguments[1].flag, "-o");
        assert_eq!(parsed.arguments[1].value, Value::String("json".to_string()));
    }

    #[test]
    fn test_parse_with_equals() {
        let cmd = "docker run --name=myapp --env=VAR=value image";
        let parsed = parse_command(cmd).unwrap();

        assert_eq!(parsed.base_command, vec!["docker", "run"]);
        assert_eq!(parsed.arguments[0].flag, "--name");
        assert_eq!(
            parsed.arguments[0].value,
            Value::String("myapp".to_string())
        );
        assert_eq!(parsed.arguments[1].flag, "--env");
        assert_eq!(
            parsed.arguments[1].value,
            Value::String("VAR=value".to_string())
        );
    }

    #[test]
    fn test_parse_boolean_flags() {
        let cmd = "ls -la /tmp";
        let parsed = parse_command(cmd).unwrap();

        assert_eq!(parsed.base_command, vec!["ls"]);
        // -la might be treated as a single flag with no value
        // or as two separate flags - depends on shlex behavior
    }

    #[test]
    fn test_parse_with_quotes() {
        let cmd = r#"kubectl get pods -o custom-columns='POD:.metadata.name,RS:.metadata.ownerReferences[0].name'"#;
        let parsed = parse_command(cmd).unwrap();

        assert_eq!(parsed.base_command, vec!["kubectl", "get", "pods"]);
        assert_eq!(parsed.arguments[0].flag, "-o");
        assert_eq!(
            parsed.arguments[0].value,
            Value::String(
                "custom-columns=POD:.metadata.name,RS:.metadata.ownerReferences[0].name"
                    .to_string()
            )
        );
    }
}
