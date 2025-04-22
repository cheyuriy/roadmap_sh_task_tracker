use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Done,
    ToDo,
    InProgress
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Done => write!(f, "Done"),
            Self::ToDo => write!(f, "ToDo"),
            Self::InProgress => write!(f, "In Progress")
        }
    }
}