use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io;
use std::process::Command;

mod help_parser;
use help_parser::{Argument, parse_help_output};

#[derive(Parser)]
#[command(name = "te")]
#[command(about = "Your helping hand for command-line interfaces", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    wrapped_command: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    History {
        #[arg(long)]
        search: Option<String>,
    },
    SavePreset {
        name: String,
    },
    ListPresets,
}

struct App {
    command_parts: Vec<String>,
    arguments: Vec<Argument>,
    list_state: ListState,
    selected_values: Vec<String>,
    preview_command: String,
    input_mode: bool,
    current_input: String,
}

impl App {
    fn new(command_parts: Vec<String>, arguments: Vec<Argument>) -> Self {
        let mut list_state = ListState::default();
        if !arguments.is_empty() {
            list_state.select(Some(0));
        }

        let selected_values = vec![String::new(); arguments.len()];
        let preview_command = Self::build_preview(&command_parts, &arguments, &selected_values);

        Self {
            command_parts,
            arguments,
            list_state,
            selected_values,
            preview_command,
            input_mode: false,
            current_input: String::new(),
        }
    }

    fn build_preview(
        command_parts: &[String],
        arguments: &[Argument],
        selected_values: &[String],
    ) -> String {
        let mut parts = command_parts.to_vec();

        for (i, arg) in arguments.iter().enumerate() {
            if !selected_values[i].is_empty() {
                parts.push(arg.name.clone());
                if !arg.takes_value {
                    continue;
                }
                parts.push(selected_values[i].clone());
            }
        }

        parts.join(" ")
    }

    fn update_preview(&mut self) {
        self.preview_command =
            Self::build_preview(&self.command_parts, &self.arguments, &self.selected_values);
    }

    fn next(&mut self) {
        if self.arguments.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.arguments.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.arguments.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.arguments.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn start_input(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.input_mode = true;
            self.current_input = self.selected_values[selected].clone();
        }
    }

    fn confirm_input(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.selected_values[selected] = self.current_input.clone();
            self.update_preview();
        }
        self.input_mode = false;
        self.current_input.clear();
    }

    fn cancel_input(&mut self) {
        self.input_mode = false;
        self.current_input.clear();
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::History { search }) => {
            handle_history(search)?;
        }
        Some(Commands::SavePreset { name }) => {
            handle_save_preset(name)?;
        }
        Some(Commands::ListPresets) => {
            handle_list_presets()?;
        }
        None => {
            if cli.wrapped_command.is_empty() {
                eprintln!("Error: No command specified");
                eprintln!("Usage: te <command> [args...]");
                std::process::exit(1);
            }

            let final_command = run_tui(cli.wrapped_command)?;
            if let Some(cmd) = final_command {
                execute_command(&cmd)?;
            }
        }
    }

    Ok(())
}

fn run_tui(command_parts: Vec<String>) -> Result<Option<String>> {
    let help_output = get_help_output(&command_parts)?;
    let arguments = parse_help_output(&help_output);

    if arguments.is_empty() {
        println!("No arguments found for this command. Executing directly...");
        return Ok(Some(command_parts.join(" ")));
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(command_parts, arguments);
    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match result {
        Ok(should_execute) => {
            if should_execute {
                Ok(Some(app.preview_command))
            } else {
                Ok(None)
            }
        }
        Err(err) => Err(err),
    }
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<bool> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(f.area());

            let title = Paragraph::new(format!(
                "te - Interactive CLI Helper | Command: {}",
                app.command_parts.join(" ")
            ))
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
            f.render_widget(title, chunks[0]);

            let items: Vec<ListItem> = app
                .arguments
                .iter()
                .enumerate()
                .map(|(i, arg)| {
                    let required_marker = if arg.required { "*" } else { " " };
                    let value_display = if app.selected_values[i].is_empty() {
                        String::from("<not set>")
                    } else {
                        app.selected_values[i].clone()
                    };

                    let content = format!(
                        "{} {} = {} | {}",
                        required_marker, arg.name, value_display, arg.description
                    );

                    ListItem::new(content)
                })
                .collect();

            let list =
                List::new(items)
                    .block(Block::default().borders(Borders::ALL).title(
                        "Arguments (↑/↓: navigate, Enter: edit, Ctrl+X: execute, ESC: cancel)",
                    ))
                    .highlight_style(
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[1], &mut app.list_state);

            if app.input_mode {
                let input = Paragraph::new(app.current_input.as_str())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Input Value (Enter: confirm, ESC: cancel)"),
                    )
                    .style(Style::default().fg(Color::Yellow));
                f.render_widget(input, chunks[2]);
            } else {
                let help_text = Paragraph::new("Press Enter to edit selected argument")
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(Color::Gray));
                f.render_widget(help_text, chunks[2]);
            }

            let preview = Paragraph::new(app.preview_command.as_str())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Command Preview"),
                )
                .style(Style::default().fg(Color::Green));
            f.render_widget(preview, chunks[3]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if app.input_mode {
                match key.code {
                    KeyCode::Enter => app.confirm_input(),
                    KeyCode::Esc => app.cancel_input(),
                    KeyCode::Char(c) => app.current_input.push(c),
                    KeyCode::Backspace => {
                        app.current_input.pop();
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') => return Ok(false),
                    KeyCode::Esc => return Ok(false),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Enter => app.start_input(),
                    KeyCode::Char('x') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        return Ok(true);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn get_help_output(command_parts: &[String]) -> Result<String> {
    if command_parts.is_empty() {
        anyhow::bail!("No command specified");
    }

    let mut cmd = Command::new(&command_parts[0]);
    for part in &command_parts[1..] {
        cmd.arg(part);
    }
    cmd.arg("--help");

    let output = cmd.output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to get help output: {}", stderr)
    }
}

fn execute_command(command_str: &str) -> Result<()> {
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

fn handle_history(search: Option<String>) -> Result<()> {
    println!("History feature not yet implemented");
    if let Some(query) = search {
        println!("Search query: {}", query);
    }
    Ok(())
}

fn handle_save_preset(name: String) -> Result<()> {
    println!("Save preset feature not yet implemented");
    println!("Preset name: {}", name);
    Ok(())
}

fn handle_list_presets() -> Result<()> {
    println!("List presets feature not yet implemented");
    Ok(())
}
