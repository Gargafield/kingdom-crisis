use std::collections::VecDeque;

use crossterm::event::{KeyCode, KeyEvent};

use crate::variable::{Variable, VariableType};

#[derive(Debug, Clone, Copy)]
pub enum Policy {
    Austerity,
    Neutral,
    Stimulus,
}

#[derive(Debug)]
pub struct App {
    pub selected_variable: VariableType,
    pub variables: [Variable; Variable::COUNT],
    pub time: f64,
    pub log: VecDeque<String>,
    pub policy: Policy,
    pub hover_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            selected_variable: VariableType::GDP,
            variables: [
                Variable::new(VariableType::GDP)
                    .grow_length(100),
                Variable::new(VariableType::Income),
                Variable::new(VariableType::Tax),
                Variable::new(VariableType::Expenses),
                Variable::new(VariableType::Opinion)
                    .grow_length(100),
                Variable::new(VariableType::Crisis)
                    .grow_length(250),
                Variable::new(VariableType::Stability)
                    .grow_length(100),
            ],
            time: 0.0,
            log: VecDeque::new(),
            policy: Policy::Neutral,
            hover_index: 0,
        }
    }

    pub fn with_start_config(mut self) -> Self {
        self.variables[VariableType::GDP as usize].value = 100.0;
        self.variables[VariableType::Income as usize].value = 0.0;
        self.variables[VariableType::Tax as usize].value = 10.0;
        self.variables[VariableType::Expenses as usize].value = 5.0;
        self.variables[VariableType::Opinion as usize].value = 100.0;
        self.variables[VariableType::Crisis as usize].value = 0.0;
        self.variables[VariableType::Stability as usize].value = 100.0;

        self
    }

    pub fn get_variable(&self, type_: &VariableType) -> &Variable {
        &self.variables[*type_ as usize]
    }

    pub fn get_variable_mut(&mut self, type_: &VariableType) -> &mut Variable {
        &mut self.variables[*type_ as usize]
    }

    pub fn log_message(&mut self, message: String) {
        self.log.push_front(message.to_string());
        if self.log.len() > 20 {
            self.log.pop_back();
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Up => {
                self.hover_index = self.hover_index.saturating_sub(1);
            }
            KeyCode::Down => {
                self.hover_index = (self.hover_index + 1).min(Variable::COUNT - 1);
            }
            KeyCode::Enter => {
                self.selected_variable = VariableType::from(self.hover_index);
            }
            _ => {}
        }
    }
}
