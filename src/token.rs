use std::fmt::Display;

use crate::acronym::Acronym;

#[derive(PartialEq, Eq, Hash)]
pub enum Token {
    Acronym(Acronym),
    Word(String),
}

impl Token {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Acronym(_) => false,
            Self::Word(word) => word.is_empty(),
        }
    }

    pub fn get_components(&self) -> Vec<String> {
        match self {
            Self::Acronym(acronym) => acronym.get_components().clone(),
            Self::Word(word) => vec![word.clone()],
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self::Word(String::new())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Acronym(a) => write!(f, "{}", a.generate_acronym()),
            Self::Word(w) => write!(f, "{w}")
        }
    }
}