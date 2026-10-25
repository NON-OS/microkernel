// NONOS Operating System (AGPL-3.0-or-later)
//! Every command the usage table documents is one the terminal runs, and a name nothing routes
//! is answered as unknown: checked on the route tables the shell and nox dispatch by, and on the
//! exit check that runs before them.

use super::exit_check::want_exit;
use super::help_usage::table::USAGE;
use super::nox_route::find::{names as nox_names, route as nox};
use super::shell_route::find::{names as shell_names, route as shell};

fn runs(name: &[u8]) -> bool {
    shell(name).is_some() || nox(name).is_some() || want_exit(&[name])
}

#[test]
fn every_documented_command_runs() {
    for (name, _, _) in USAGE.iter().flat_map(|g| g.iter()) {
        assert!(runs(name), "help documents {} and nothing runs it", String::from_utf8_lossy(name));
    }
}

#[test]
fn a_name_nothing_routes_is_answered_as_unknown() {
    for name in [&b"no-such-command"[..], b"", b"LS", b"ls ", b"lsx", b"exit2"] {
        assert!(!runs(name), "{:?} runs", String::from_utf8_lossy(name));
    }
}

/// Every name a table lists routes through it, and none is listed twice.
fn held(all: &[&[u8]], routes: fn(&[u8]) -> bool) {
    for (i, n) in all.iter().enumerate() {
        assert!(routes(n), "{} is listed and does not route", String::from_utf8_lossy(n));
        assert!(!all[..i].contains(n), "{} is listed twice", String::from_utf8_lossy(n));
    }
}

#[test]
fn every_listed_name_routes_and_each_once() {
    held(&shell_names().collect::<Vec<_>>(), |n| shell(n).is_some());
    held(&nox_names().collect::<Vec<_>>(), |n| nox(n).is_some());
}
