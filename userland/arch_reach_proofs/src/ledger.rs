// NONOS Operating System (AGPL-3.0-or-later)
//! A ledger is rows of two words, a key and a value, each followed by one or
//! more indented lines saying why. Blank lines and lines starting `#` are
//! comments. Rows are sorted by key, so a key appears once.

/// One row: its key, its value, and its reason joined into one line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub key: String,
    pub value: String,
    pub reason: String,
}

/// Why a ledger is refused, with the line (from 1) or the key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    TooLong(usize),
    ReasonWithoutRow(usize),
    BadRow(usize),
    Unsorted(usize),
    Duplicate(usize),
    NoReason(String),
}

/// The house limit on a line's length, in characters.
pub const MAX_LINE: usize = 100;

pub fn parse(text: &str) -> Result<Vec<Row>, LedgerError> {
    let mut rows: Vec<Row> = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let at = i + 1;
        if raw.chars().count() > MAX_LINE {
            return Err(LedgerError::TooLong(at));
        }
        if raw.trim().is_empty() || raw.starts_with('#') {
            continue;
        }
        if raw.starts_with(char::is_whitespace) {
            let row = rows.last_mut().ok_or(LedgerError::ReasonWithoutRow(at))?;
            if !row.reason.is_empty() {
                row.reason.push(' ');
            }
            row.reason.push_str(raw.trim());
            continue;
        }
        let mut words = raw.split_whitespace();
        let (Some(key), Some(value), None) = (words.next(), words.next(), words.next()) else {
            return Err(LedgerError::BadRow(at));
        };
        if let Some(last) = rows.last() {
            if last.reason.is_empty() {
                return Err(LedgerError::NoReason(last.key.clone()));
            }
            if last.key.as_str() == key {
                return Err(LedgerError::Duplicate(at));
            }
            if last.key.as_str() > key {
                return Err(LedgerError::Unsorted(at));
            }
        }
        rows.push(Row { key: key.into(), value: value.into(), reason: String::new() });
    }
    match rows.last() {
        Some(last) if last.reason.is_empty() => Err(LedgerError::NoReason(last.key.clone())),
        _ => Ok(rows),
    }
}
