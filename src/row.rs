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
