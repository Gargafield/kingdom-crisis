use std::collections::vec_deque;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum VariableType {
    #[default]
    GDP,
    Income,
    Tax,
    Expenses,
    Opinion,
    Crisis,
    Stability,
}

impl From<usize> for VariableType {
    fn from(value: usize) -> Self {
        match value {
            0 => VariableType::GDP,
            1 => VariableType::Income,
            2 => VariableType::Tax,
            3 => VariableType::Expenses,
            4 => VariableType::Opinion,
            5 => VariableType::Crisis,
            6 => VariableType::Stability,
            _ => VariableType::GDP,
        }
    }
}

#[derive(Debug)]
pub struct VariableDisplay {
    pub emoji: char,
    pub name: &'static str,
}

const VARIABLE_COUNT: usize = 7;
const VARIABLE_DISPLAY: [VariableDisplay; VARIABLE_COUNT] = [
    VariableDisplay {
        emoji: '💲',
        name: "GDP",
    },
    VariableDisplay {
        emoji: '💰',
        name: "Income",
    },
    VariableDisplay {
        emoji: '🏧',
        name: "Tax",
    },
    VariableDisplay {
        emoji: '💸',
        name: "Expenses",
    },
    VariableDisplay {
        emoji: '😁',
        name: "Opinion",
    },
    VariableDisplay {
        emoji: '🔥',
        name: "Crisis",
    },
    VariableDisplay {
        emoji: '🏦',
        name: "Stability",
    },
];

#[derive(Debug)]
pub struct Variable {
    pub type_: VariableType,
    pub display: &'static VariableDisplay,
    pub last_values: vec_deque::VecDeque<f64>,
    pub value: f64,
    pub max_length: usize,
}

impl Variable {
    pub const COUNT: usize = VARIABLE_COUNT;

    pub fn new(type_: VariableType) -> Self {        
        Self {
            type_,
            display: &VARIABLE_DISPLAY[type_ as usize],
            last_values: vec_deque::VecDeque::with_capacity(10),
            value: 0.0,
            max_length: 10,
        }
    }

    pub fn grow_length(mut self, new_length: usize) -> Self {
        self.last_values = vec_deque::VecDeque::with_capacity(new_length);
        self.max_length = new_length;
        self
    }

    pub fn update(&mut self, value: f64) {
        self.value = value;
        self.last_values.push_back(value);

        if self.last_values.len() > self.max_length {
            self.last_values.pop_front();
        }
    }

    pub fn change(&self) -> f64 {
        if self.last_values.len() < 2 {
            return 0.0;
        }

        let first = self.last_values.front().unwrap();
        let last = self.last_values.back().unwrap();

        last - first
    }
}
