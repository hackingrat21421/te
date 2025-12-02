use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    Terminal, TerminalOptions, Viewport,
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};
use std::fs::OpenOptions;

use crate::app::{App, Value};
use crate::command_parser::parse_command;

pub fn run_tui(command_str: String) -> Result<Option<String>> {
    let parsed = parse_command(&command_str)?;

    if parsed.arguments.is_empty() {
        println!("No arguments to edit. Command: {}", command_str);
        return Ok(Some(command_str));
    }

    enable_raw_mode()?;

    // Open /dev/tty directly for both reading and writing (like fzf does)
    // This allows the TUI to work inside command substitution
    let mut tty = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")?;

    execute!(tty, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(tty);
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Fullscreen,
        },
    )?;

    let mut app = App::new(parsed.base_command, parsed.arguments);
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
            let area = f.area();

            // Help text on first line
            let help_area = ratatui::layout::Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: 1,
            };
            let help = Paragraph::new(
                "↑/↓: navigate, Space: toggle, Enter: edit, Ctrl+X: execute, ESC: cancel",
            )
            .style(Style::default().fg(Color::DarkGray));
            f.render_widget(help, help_area);

            // Render each argument
            let selected = app.list_state.selected().unwrap_or(0);
            for (i, arg) in app.arguments.iter().enumerate() {
                let row_area = ratatui::layout::Rect {
                    x: area.x,
                    y: area.y + 1 + i as u16,
                    width: area.width,
                    height: 1,
                };

                // Argument name (max 20 chars)
                let name_display = if arg.flag.is_empty() {
                    "(positional)".to_string()
                } else {
                    arg.flag.clone()
                };
                let name_display = if name_display.len() > 20 {
                    format!("{}...", &name_display[..17])
                } else {
                    name_display
                };

                // Apply style based on selection
                let (name_style, value_style) = if i == selected {
                    (
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                        Style::default().fg(Color::White).bg(Color::DarkGray),
                    )
                } else {
                    (
                        Style::default().fg(Color::Gray),
                        Style::default().fg(Color::White),
                    )
                };

                // Name area (left 20 chars)
                let name_area = ratatui::layout::Rect {
                    x: row_area.x,
                    y: row_area.y,
                    width: 20,
                    height: 1,
                };

                let name_widget = Paragraph::new(name_display).style(name_style);
                f.render_widget(name_widget, name_area);

                // Value display (right side) depends on Value type
                match &arg.value {
                    Value::Checked(checked) => {
                        // Layout: [name 20 chars] [checkbox flex]
                        let checkbox_area = ratatui::layout::Rect {
                            x: row_area.x + 20,
                            y: row_area.y,
                            width: row_area.width.saturating_sub(20),
                            height: 1,
                        };

                        let display = if *checked { "TRUE" } else { "FALSE" };
                        let checkbox_widget = Paragraph::new(display).style(value_style);
                        f.render_widget(checkbox_widget, checkbox_area);
                    }
                    Value::String(s) => {
                        let display = if app.input_mode && i == selected {
                            app.current_input.as_str()
                        } else {
                            s.as_str()
                        };
                        // Layout: [name 20 chars] [value flex]
                        let value_area = ratatui::layout::Rect {
                            x: row_area.x + 20,
                            y: row_area.y,
                            width: row_area.width.saturating_sub(20),
                            height: 1,
                        };

                        let value_widget = Paragraph::new(display).style(value_style);
                        f.render_widget(value_widget, value_area);
                    }
                };
            }
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
                    KeyCode::Char(' ') => app.toggle_checkbox(),
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
