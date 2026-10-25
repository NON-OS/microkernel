// NONOS Operating System (AGPL-3.0-or-later)
//! What the screen shows: cells counted in characters, and the scroll window.

use crate::line_window::{cells_of, fit_cells, window};

// What the screen shows.

#[test]
fn cells_count_characters_not_bytes() {
    assert_eq!(cells_of("café".as_bytes()), 4);
    assert_eq!(cells_of("日本".as_bytes()), 4, "wide characters take two cells");
    assert_eq!(fit_cells("日本語".as_bytes(), 5), "日本".len(), "a half fitting one stays out");
}

#[test]
fn the_cursor_cell_is_counted_in_characters() {
    let body = "éé€x".as_bytes();
    // After three characters (seven bytes), the cursor is in the fourth cell.
    let (start, stop, cell) = window(body, 7, 20);
    assert_eq!((start, stop, cell), (0, body.len(), 3));
}

#[test]
fn a_long_line_scrolls_by_cells_and_keeps_the_cursor_on_screen() {
    let line: String = "é".repeat(30);
    let body = line.as_bytes();
    let (start, stop, cell) = window(body, body.len(), 10);
    assert_eq!(cell, 9, "the cursor in the last cell");
    assert_eq!(&body[start..stop], "é".repeat(9).as_bytes(), "nine characters before it");
    assert!(core::str::from_utf8(&body[start..stop]).is_ok());
}

#[test]
fn a_wide_character_cut_by_the_left_edge_is_left_out() {
    let body = "a日本語x".as_bytes();
    // Cursor at the end: 8 cells to the cursor, 5 on screen, scroll by 4.
    let (start, stop, cell) = window(body, body.len(), 5);
    assert!(core::str::from_utf8(&body[start..stop]).is_ok());
    assert_eq!(&body[start..stop], "語x".as_bytes());
    assert_eq!(cell, 3);
}
