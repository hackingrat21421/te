use anyhow::Result;
use std::process::Command;

pub fn execute_command(command_str: &str) -> Result<()> {
    println!("\nExecuting: {}\n", command_str);

    let parts: Vec<&str> = command_str.split_whitespace().collect();
    if parts.is_empty() {
        anyhow::bail!("Empty command");
    }

    let status = Command::new(parts[0]).args(&parts[1..]).status()?;

    if !status.success() {
        eprintln!("\nCommand exited with status: {}", status);
    }

    Ok(())
}
