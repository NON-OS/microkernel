// NONOS Operating System (AGPL-3.0-or-later)
//! Tab's list: aligned columns filled down then across, a few rows at most
//! with the rest counted, a path's last part, and a place beside the prompt.

use super::complete_list::{cell, grid, label, top, Grid, GUTTER, MAX_ROWS};

#[test]
fn columns_are_as_wide_as_the_longest_name() {
    let g = grid(&[4, 8, 6], 40);
    assert_eq!(g, Grid { width: 8 + GUTTER, cols: 4, rows: 1, shown: 3 });
    let g = grid(&[4, 8, 6, 3, 5], 20);
    assert_eq!((g.cols, g.rows, g.shown), (2, 3, 5));
}

#[test]
fn a_long_list_stops_at_the_row_cap_and_counts_the_rest() {
    let g = grid(&[3; 100], 10);
    assert_eq!((g.cols, g.rows), (2, MAX_ROWS));
    assert_eq!(g.shown, 2 * MAX_ROWS, "{} more are counted", 100 - g.shown);
}

#[test]
fn a_name_wider_than_the_window_still_gets_a_column() {
    assert_eq!(grid(&[90], 40).cols, 1);
}

#[test]
fn names_fill_down_then_across() {
    let g = grid(&[3, 3, 3, 3, 3], 10);
    assert_eq!((g.cols, g.rows), (2, 3));
    let at: Vec<_> = (0..5).map(|i| cell(&g, i)).collect();
    assert_eq!(at, [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1)]);
}

#[test]
fn a_path_shows_its_last_part() {
    assert_eq!(label(b"/home/nonos/workspace/"), b"workspace/");
    assert_eq!(label(b"/home/nonos/readme.txt"), b"readme.txt");
    assert_eq!(label(b"capsules"), b"capsules");
    assert_eq!(label(b"/"), b"/");
}

#[test]
fn the_list_sits_under_the_prompt_or_over_the_body_above_it() {
    assert_eq!(top(100, 20, 3, 40, 400), 120, "room below the prompt");
    assert_eq!(top(380, 20, 3, 40, 400), 320, "no room below: just above it");
    assert_eq!(top(60, 20, 8, 40, 100), 40, "never above the body");
}
