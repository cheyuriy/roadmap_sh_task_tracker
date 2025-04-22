use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_string_representation() {
        let expected = vec!["Done", "ToDo", "In Progress"];
        let statuses = vec![Status::Done, Status::ToDo, Status::InProgress];
        for (s, e) in statuses.iter().zip(expected) {
            assert_eq!(s.to_string(), e.to_string());
        }
    }
}