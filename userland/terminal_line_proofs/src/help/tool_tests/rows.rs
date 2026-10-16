// NONOS Operating System (AGPL-3.0-or-later)
//! The tools row names each program once, and the splash names the build's architecture.

use crate::help_rows::help_tables;

/// The tools row names each program once, however many names it answers to.
#[test]
fn the_tools_row_names_each_program_once() {
    use crate::tool_list::tool_list;
    let tools: [(&[u8], &[u8]); 3] =
        [(b"rg", b"ripgrep"), (b"ripgrep", b"ripgrep"), (b"fd", b"fd")];
    assert_eq!(tool_list(&tools), b"rg  fd".to_vec());
    let row = help_tables()[0].1.last().unwrap().clone();
    let mut names: Vec<&str> = row.split_whitespace().skip(1).collect();
    let n = names.len();
    names.dedup();
    assert!(n >= 5 && names.len() == n, "{row:?}");
}

/// The splash's arch row is the architecture the terminal was built for, not
/// a fixed "x86_64".
#[test]
fn the_splash_names_the_architecture_it_was_built_for() {
    assert_eq!(crate::fetch_arch::arch(), std::env::consts::ARCH);
    let src = include_str!("../../../../capsule_terminal/src/paint/fetch.rs");
    assert!(src.contains("fetch_arch::arch()") && !src.contains("\"arch\", \"x86_64\""));
}
