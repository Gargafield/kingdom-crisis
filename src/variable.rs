use std::collections::vec_deque;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum VariableRepresentation {
    #[default]
    Value,
    Currency,
    Percentage,
}

impl VariableRepresentation {
    pub fn get_suffix(&self) -> &'static str {
        match self {
            VariableRepresentation::Value => "",
            VariableRepresentation::Currency => "",
            VariableRepresentation::Percentage => "%",
        }
    }

    pub fn get_prefix(&self) -> &'static str {
        match self {
            VariableRepresentation::Value => "",
            VariableRepresentation::Currency => "$",
            VariableRepresentation::Percentage => "",
        }
    }
}

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
    pub representation: VariableRepresentation,
}

const VARIABLE_COUNT: usize = 7;
const VARIABLE_DISPLAY: [VariableDisplay; VARIABLE_COUNT] = [
    VariableDisplay {
        emoji: '💲',
        name: "GDP",
        representation: VariableRepresentation::Currency,
    },
    VariableDisplay {
        emoji: '💰',
        name: "Income",
        representation: VariableRepresentation::Currency,
    },
    VariableDisplay {
        emoji: '🏧',
        name: "Tax",
        representation: VariableRepresentation::Percentage,
    },
    VariableDisplay {
        emoji: '💸',
        name: "Expenses",
        representation: VariableRepresentation::Currency,
    },
    VariableDisplay {
        emoji: '😁',
        name: "Opinion",
        representation: VariableRepresentation::Percentage,
    },
    VariableDisplay {
        emoji: '🔥',
        name: "Crisis",
        representation: VariableRepresentation::Value,
    },
    VariableDisplay {
        emoji: '🏦',
        name: "Stability",
        representation: VariableRepresentation::Percentage,
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
            last_values: vec_deque::VecDeque::with_capacity(50),
            value: 0.0,
            max_length: 50,
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
