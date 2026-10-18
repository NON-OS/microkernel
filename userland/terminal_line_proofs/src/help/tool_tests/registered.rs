// NONOS Operating System (AGPL-3.0-or-later)
//! Every tool the terminal offers, and every one the nox index names, is one the image runs.

use crate::help_rows::tool_table;

/*
 * Every tool the terminal offers is one the kernel registers. The kernel's
 * list is generated from apps.list, so a service missing there is a name
 * that answers "not installed" in every build. `linux` is the one the kernel
 * runs itself, the Linux personality, so it must be named in the kernel's
 * registry instead.
 */
#[test]
fn every_tool_the_terminal_offers_is_registered() {
    let apps = include_str!("../../../../apps.list");
    let kernel = include_str!("../../../../../src/userspace/tool_capsules/registry.rs");
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

/// The nox index names the tools from the table that runs them, not a fixed
/// line: it once offered sd and tokio-smoke, which the standard image does
/// not carry, and none of the seven tools it does.
#[test]
fn the_nox_index_names_the_tools_the_image_runs() {
    let src = include_str!("../../../../capsule_terminal/src/command/builtin/nox/help.rs");
    assert!(src.contains("tool_list(TOOLS)"), "the row is built from the tool table");
    assert!(!src.contains("tokio-smoke") && !src.contains("sd /"), "no tool the image lacks");
    let owned = tool_table();
    let tools: Vec<(&[u8], &[u8])> =
        owned.iter().map(|(a, b)| (a.as_slice(), b.as_slice())).collect();
    let row = String::from_utf8(crate::tool_list::tool_list(&tools)).unwrap();
    for name in ["grex", "tokei", "csview"] {
        assert!(row.split_whitespace().any(|w| w == name), "{name} missing from {row:?}");
    }
}
