use ratatui::widgets::ListState;
use crate::help_parser::Argument;

pub struct App {
    pub command_parts: Vec<String>,
    pub arguments: Vec<Argument>,
    pub list_state: ListState,
    pub selected_values: Vec<String>,
    pub preview_command: String,
    pub input_mode: bool,
    pub current_input: String,
}

impl App {
    pub fn new(command_parts: Vec<String>, arguments: Vec<Argument>) -> Self {
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

    pub fn update_preview(&mut self) {
        self.preview_command =
            Self::build_preview(&self.command_parts, &self.arguments, &self.selected_values);
    }

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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

    pub fn start_input(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.input_mode = true;
            self.current_input = self.selected_values[selected].clone();
        }
    }

    pub fn confirm_input(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.selected_values[selected] = self.current_input.clone();
            self.update_preview();
        }
        self.input_mode = false;
        self.current_input.clear();
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = false;
        self.current_input.clear();
    }
}
