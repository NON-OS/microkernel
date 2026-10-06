// NONOS Operating System (AGPL-3.0-or-later)
//! The tools the terminal offers by name, against what the kernel runs.

use crate::help_rows::{help_tables, tool_table};

/*
 * Every tool the terminal offers is one the kernel registers. The kernel's
 * list is generated from apps.list, so a service missing there is a name
 * that answers "not installed" in every build. `linux` is the one the kernel
 * runs itself, the Linux personality, so it must be named in the kernel's
 * registry instead.
 */
#[test]
fn every_tool_the_terminal_offers_is_registered() {
    let apps = include_str!("../../apps.list");
    let kernel = include_str!("../../../src/userspace/tool_capsules/registry.rs");
    let registered: Vec<&str> = apps
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .filter_map(|l| l.split_whitespace().next())
        .collect();
    let tools = tool_table();
    assert!(!tools.is_empty(), "parsed no tools");
    for (typed, service) in tools {
        let service = String::from_utf8(service).unwrap();
        let typed = String::from_utf8(typed).unwrap();
        if service == "linux" {
            assert!(kernel.contains("b\"tool.linux\""), "the kernel runs no tool.linux");
            continue;
        }
        assert!(
            registered.contains(&service.as_str()),
            "{typed} runs tool.{service}, which apps.list does not register"
        );
    }
}

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

/// The nox index names the tools from the table that runs them, not a fixed
/// line: it once offered sd and tokio-smoke, which the standard image does
/// not carry, and none of the seven tools it does.
#[test]
fn the_nox_index_names_the_tools_the_image_runs() {
    let src = include_str!("../../capsule_terminal/src/command/builtin/nox/help.rs");
    assert!(src.contains("tool_list(TOOLS)"), "the row is built from the tool table");
    assert!(!src.contains("tokio-smoke") && !src.contains("sd /"), "no tool the image lacks");
    let owned = tool_table();
    let tools: Vec<(&[u8], &[u8])> = owned.iter().map(|(a, b)| (a.as_slice(), b.as_slice())).collect();
    let row = String::from_utf8(crate::tool_list::tool_list(&tools)).unwrap();
    for name in ["grex", "tokei", "csview"] {
        assert!(row.split_whitespace().any(|w| w == name), "{name} missing from {row:?}");
    }
}

/// The splash's arch row is the architecture the terminal was built for, not
/// a fixed "x86_64".
#[test]
fn the_splash_names_the_architecture_it_was_built_for() {
    assert_eq!(crate::fetch_arch::arch(), std::env::consts::ARCH);
    let src = include_str!("../../capsule_terminal/src/paint/fetch.rs");
    assert!(src.contains("fetch_arch::arch()") && !src.contains("\"arch\", \"x86_64\""));
}
