
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    #[default]
    Austerity,
    Neutral,
    Stimulus
}

impl From<usize> for ActionType {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Austerity,
            1 => Self::Neutral,
            2 => Self::Stimulus,
            _ => Self::default(),
        }
    }
}

#[derive(Debug)]
pub struct ActionDisplay {
    pub emoji: char,
    pub name: &'static str,
}

const ACTION_COUNT: usize = 3;
const ACTION_DISPLAY: [ActionDisplay; ACTION_COUNT] = [
    ActionDisplay {
        emoji: '🫴',
        name: "Austerity",
    },
    ActionDisplay {
        emoji: '➖',
        name: "Neutral",
    },
    ActionDisplay {
        emoji: '💸',
        name: "Stimulus",
    },
];

#[derive(Debug)]
pub struct Action {
    pub type_: ActionType,
    pub display: &'static ActionDisplay,
}

impl Action {
    pub const COUNT: usize = ACTION_COUNT;

    pub fn new(type_: ActionType) -> Self {        
        Self {
            type_,
            display: &ACTION_DISPLAY[type_ as usize],
        }
    }
}
