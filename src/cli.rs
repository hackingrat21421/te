use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "te")]
#[command(about = "Your helping hand for command-line interfaces", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub wrapped_command: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    History {
        #[arg(long)]
        search: Option<String>,
    },
    SavePreset {
        name: String,
    },
    ListPresets,
}
