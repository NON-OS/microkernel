// NONOS Operating System (AGPL-3.0-or-later)
//! The counts line, the name offsets, and which nodes are directories.

use super::paths;
use crate::tree_render::{counts, render};

#[test]
fn counts_separate_directories_from_leaves() {
    // a/ and a/b/ hold things; a/b/c and d do not.
    let (dirs, files) = counts(&render(&paths(&["a/b/c", "d"])));
    assert_eq!((dirs, files), (2, 2));
}

#[test]
fn counts_of_a_flat_listing_are_all_files() {
    let (dirs, files) = counts(&render(&paths(&["x", "y", "z"])));
    assert_eq!((dirs, files), (0, 3));
}

/// The colouring the command does depends on `name_at` pointing exactly at the
/// name, so the connectors keep their own colour and the name takes the
/// directory blue.
#[test]
fn the_name_offset_points_past_the_connectors() {
    for row in render(&paths(&["a/b/c", "d"])) {
        let line = String::from_utf8(row.line.clone()).unwrap();
        let name = &line[row.name_at..];
        assert!(!name.is_empty(), "empty name in {line:?}");
        assert!(
            !name.starts_with([' ', '|', '`', '-']),
            "name offset landed on a connector in {line:?}"
        );
        assert!(name.chars().all(|c| c.is_ascii_alphanumeric()), "{name:?}");
    }
}

/// Everything with something under it is a directory, and nothing else is.
#[test]
fn only_nodes_with_children_are_marked_as_directories() {
    let rows = render(&paths(&["src/main.rs", "README"]));
    let dirs: Vec<String> = rows
        .iter()
        .filter(|r| r.is_dir)
        .map(|r| String::from_utf8(r.line[r.name_at..].to_vec()).unwrap())
        .collect();
    assert_eq!(dirs, vec!["src"]);
}
