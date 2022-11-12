use std::fs;
use crate::row::Row;

pub(super) struct File {
    pub(super) lines: Vec<Row>,
    pub(super) dirty: bool,
}

pub(super) fn load_file(path: &str) -> File {
    let buffer = fs::read_to_string(path);

    if buffer.is_err() {
        return File { lines: Vec::new(), dirty: false };
    }

    let temp = buffer.unwrap();

    let lines: Vec<Row> = temp
        .split('\n')
        .into_iter()
        .map(Row::from)
        .collect();

    File { lines, dirty: false }
}

impl File {

}