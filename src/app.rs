use ratatui::widgets::ListState;

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub flag: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Checked(bool),
}

pub struct App {
    pub base_command: Vec<String>,
    pub arguments: Vec<Argument>,
    pub list_state: ListState,
    pub preview_command: String,
    pub input_mode: bool,
    pub current_input: String,
}

impl App {
    pub fn new(base_command: Vec<String>, arguments: Vec<Argument>) -> Self {
        let mut list_state = ListState::default();
        if !arguments.is_empty() {
            list_state.select(Some(0));
        }

        let preview_command = Self::build_preview(&base_command, &arguments);

        Self {
            base_command,
            arguments,
            list_state,
            preview_command,
            input_mode: false,
            current_input: String::new(),
        }
    }

    fn build_preview(base_command: &[String], arguments: &[Argument]) -> String {
        let mut parts = base_command.to_vec();

        for arg in arguments.iter() {
            match &arg.value {
                Value::String(s) => {
                    if !arg.flag.is_empty() {
                        parts.push(arg.flag.clone());
                    }
                    parts.push(s.clone());
                }
                Value::Checked(checked) => {
                    if *checked {
                        parts.push(arg.flag.clone());
                    }
                }
            }
        }

        parts.join(" ")
    }

    pub fn update_preview(&mut self) {
        self.preview_command = Self::build_preview(&self.base_command, &self.arguments);
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
            // Only allow editing for String values
            if let Value::String(s) = &self.arguments[selected].value {
                self.input_mode = true;
                self.current_input = s.clone();
            }
        }
    }

    pub fn confirm_input(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.arguments[selected].value = Value::String(self.current_input.clone());
            self.update_preview();
        }
        self.input_mode = false;
        self.current_input.clear();
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = false;
        self.current_input.clear();
    }

    pub fn toggle_checkbox(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if let Value::Checked(checked) = &self.arguments[selected].value {
                self.arguments[selected].value = Value::Checked(!checked);
                self.update_preview();
            }
        }
    }
}
