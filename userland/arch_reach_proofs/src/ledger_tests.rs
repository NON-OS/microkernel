// NONOS Operating System (AGPL-3.0-or-later)
//! A ledger that cannot say why is refused, however it fails to.

use crate::ledger::{parse, LedgerError, MAX_LINE};

#[test]
fn rows_take_every_reason_line() {
    let rows = parse("# note\n\na 1\n    one\n    two\nb 2\n\tthree\n").unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].reason, "one two");
    assert_eq!((rows[1].key.as_str(), rows[1].value.as_str()), ("b", "2"));
}

#[test]
fn a_row_without_a_reason_is_refused() {
    assert_eq!(parse("a 1\nb 1\n    why\n"), Err(LedgerError::NoReason("a".into())));
    assert_eq!(parse("a 1\n    why\nb 1\n"), Err(LedgerError::NoReason("b".into())));
}

#[test]
fn a_reason_before_any_row_is_refused() {
    assert_eq!(parse("    why\na 1\n"), Err(LedgerError::ReasonWithoutRow(1)));
}

#[test]
fn a_row_of_one_or_three_words_is_refused() {
    assert_eq!(parse("a\n    why\n"), Err(LedgerError::BadRow(1)));
    assert_eq!(parse("a 1 x\n    why\n"), Err(LedgerError::BadRow(1)));
}

#[test]
fn unsorted_and_twice_listed_rows_are_refused() {
    assert_eq!(parse("b 1\n    why\na 1\n    why\n"), Err(LedgerError::Unsorted(3)));
    assert_eq!(parse("a 1\n    why\na 2\n    why\n"), Err(LedgerError::Duplicate(3)));
}

#[test]
fn a_line_past_the_house_limit_is_refused() {
    let long = format!("a 1\n    {}\n", "w".repeat(MAX_LINE - 3));
    assert_eq!(parse(&long), Err(LedgerError::TooLong(2)));
    assert!(parse(&format!("a 1\n    {}\n", "w".repeat(MAX_LINE - 4))).is_ok());
}
