use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub(super) struct Row {
    pub(super) content: String,
}

impl From<&str> for Row {
    fn from(x: &str) -> Self {
        Self {
            content: x.to_string(),
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}