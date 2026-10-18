// NONOS Operating System (AGPL-3.0-or-later)
//! The drawn shape: order, trunks, levels, and the depth bound.

use super::drawn;
use crate::tree_render::MAX_DEPTH;

#[test]
fn a_flat_directory_lists_in_order_with_the_last_one_closed() {
    assert_eq!(drawn(&["b", "a", "c"]), vec!["|-- a", "|-- b", "`-- c"]);
}

#[test]
fn one_entry_is_the_last_entry() {
    assert_eq!(drawn(&["only"]), vec!["`-- only"]);
}

#[test]
fn nothing_draws_nothing() {
    assert!(drawn(&[]).is_empty());
}

/// A directory that only appears as part of a longer path still gets a row.
#[test]
fn intermediate_directories_are_drawn_even_when_not_listed() {
    assert_eq!(drawn(&["src/main.rs"]), vec!["`-- src", "    `-- main.rs"]);
}

/// The line under a node must continue while that node has later siblings and
/// stop when it does not. This is the rule that produces branches leading
/// nowhere when it is wrong.
#[test]
fn the_trunk_continues_only_while_there_are_later_siblings() {
    assert_eq!(drawn(&["a/x", "b/y"]), vec!["|-- a", "|   `-- x", "`-- b", "    `-- y"]);
}

#[test]
fn deep_nesting_indents_once_per_level() {
    let out = drawn(&["a/b/c/d"]);
    assert_eq!(out, vec!["`-- a", "    `-- b", "        `-- c", "            `-- d"]);
}

#[test]
fn siblings_at_two_levels_keep_their_own_trunks() {
    assert_eq!(drawn(&["a/x", "a/y", "b"]), vec!["|-- a", "|   |-- x", "|   `-- y", "`-- b"]);
}

/// Awkward input from a filesystem rather than from a test: doubled and
/// trailing separators must not invent empty levels.
#[test]
fn repeated_and_trailing_separators_do_not_create_empty_levels() {
    assert_eq!(drawn(&["a//b/"]), vec!["`-- a", "    `-- b"]);
}

#[test]
fn a_path_listed_twice_is_drawn_once() {
    assert_eq!(drawn(&["a/b", "a/b"]), vec!["`-- a", "    `-- b"]);
}

#[test]
fn depth_is_bounded() {
    let deep: String = (0..MAX_DEPTH + 10).map(|i| format!("d{i}/")).collect();
    let out = drawn(&[&deep]);
    let deepest = out
        .iter()
        .map(|l| l.len() - l.trim_start_matches([' ', '|', '`', '-']).len())
        .max()
        .unwrap();
    assert!(out.len() <= MAX_DEPTH, "drew {} levels", out.len());
    assert!(deepest <= MAX_DEPTH * 4);
}
