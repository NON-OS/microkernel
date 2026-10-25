// NONOS Operating System (AGPL-3.0-or-later)
//! `help` as it renders: its width, its columns, and the tools it names.

use crate::help_rows::{help_rows, help_tables};

/// `help` is the first thing anyone runs. A row wider than the terminal wraps,
/// the columns stop lining up, and the screen that is supposed to orient a new
/// reader is the one that looks broken.
/// Eighty columns, not the 96-column buffer.
///
/// This test used to assert against `COLS`, the width of the line buffer, and
/// passed while `help` was visibly clipped in a default window. The buffer is
/// not the viewport. Eighty is the width every terminal has defaulted to for
/// forty years and the one the window manifest now opens at.
#[test]
fn no_help_row_is_wider_than_the_terminal() {
    const COLS: usize = 80;
    let rows = help_rows();
    assert!(rows.len() >= 15, "only parsed {} help rows", rows.len());
    for row in rows {
        assert!(row.len() <= COLS, "{} cols: {row:?}", row.len());
    }
}

/*
 * Each row is a label under the same indent with its text in one column
 * down the table. A row whose text starts somewhere else is a row nobody
 * can scan.
 */
#[test]
fn every_help_row_is_a_label_or_a_continuation() {
    for (table, rows) in help_tables() {
        let column = |r: &str| {
            let after = r[2..].find(' ').map(|i| i + 2).unwrap_or(r.len());
            after + r[after..].len() - r[after..].trim_start().len()
        };
        let first = column(&rows[0]);
        for row in &rows {
            assert!(
                row.starts_with("  ") && row[2..].starts_with(|c: char| c.is_ascii_lowercase()),
                "{table}: unlabelled row {row:?}"
            );
            assert_eq!(column(row), first, "{table}: text out of column in {row:?}");
        }
    }
}
