// NONOS Operating System (AGPL-3.0-or-later)
//! `help` names a command only when `help <command>` has a page for it and the
//! terminal dispatches it, and no page promises what the command does not do.
//! Read from the usage table and the route tables as the capsule holds them.

use super::help_usage::table::USAGE;
use super::nox_route::find::route as nox;
use super::shell_route::find::route as shell;
use crate::help_layout::GROUPS;

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

/// Every name the usage table has a page for, in table order.
fn usage_names() -> Vec<String> {
    USAGE.iter().flat_map(|g| g.iter()).map(|(n, _, _)| text(n)).collect()
}

fn grouped() -> Vec<String> {
    GROUPS
        .iter()
        .flat_map(|(_, list)| std::str::from_utf8(list).unwrap().split_whitespace())
        .map(str::to_string)
        .collect()
}

#[test]
fn every_command_help_lists_has_a_usage_page() {
    let names = usage_names();
    assert!(names.len() >= 60, "parsed {} entries", names.len());
    for name in grouped() {
        assert!(names.contains(&name), "help lists {name} and help {name} has no page");
    }
}

/// Answered by the shell's route or by nox's, as the capsule routes it.
fn dispatched(name: &str) -> bool {
    shell(name.as_bytes()).is_some() || nox(name.as_bytes()).is_some()
}

#[test]
fn every_command_help_lists_runs_on_its_own() {
    for name in grouped() {
        assert!(dispatched(&name), "help lists {name} and nothing dispatches it");
    }
}

#[test]
fn no_page_promises_what_the_command_does_not_do() {
    let pages: Vec<String> = USAGE
        .iter()
        .flat_map(|g| g.iter())
        .map(|(_, u, w)| format!("{} {}", text(u), text(w)))
        .collect();
    let says = |words: &str| pages.iter().any(|p| p.contains(words));
    assert!(!says("or update their time"), "touch updates no time");
    assert!(!says("let a stopped job carry on"), "nothing is ever stopped");
    assert!(!says("the registered services and the pids"), "service takes one name");
    assert!(!pages.iter().any(|p| p.ends_with("no argument goes home")), "cd's home is $HOME");
    assert!(!says("yet"));
}
