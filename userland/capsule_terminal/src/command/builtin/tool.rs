// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Run one of the baked, attested command-line tools (grex, tokei, csview, ...)
//! from the shell. The kernel spawns the tool parented to this terminal, so the
//! same async drain job that streams a store install's output streams the
//! tool's stdout here. Adding a tool is one line in `TOOLS`.

use alloc::vec::Vec;
use nonos_libc::mk_tool_run;

use crate::command::builtin::nox::install::InstallJob;
use crate::term::state::State;

/// The installed command-line tools: what you type, and the service it runs.
///
/// The typed name comes first because it need not be the service's: a tool
/// can keep the name its users have in their fingers without renaming its
/// capsule. Every service here is one `userland/apps.list` registers, which
/// terminal_line_proofs checks, so a name on this list always has a program
/// behind it.
pub const TOOLS: &[(&[u8], &[u8])] = &[
    (b"grex", b"grex"),
    (b"dotenv-linter", b"dotenv-linter"),
    (b"pastel", b"pastel"),
    (b"jsonxf", b"jsonxf"),
    (b"tokei", b"tokei"),
    (b"huniq", b"huniq"),
    (b"csview", b"csview"),
];

// `install` is absent for the same reason as `sd`: the builtin that installs
// from the market answers to the name first (jobs::classify).
// `sd` is deliberately absent. It runs from the vfs store through `STORE_TOOLS`
// in jobs::classify, which is checked before this table, so an entry here would
// never be reached and would read as a second answer to the same question.

/// The service a typed name runs, or `None` if no tool answers to it.
pub fn service_for(name: &[u8]) -> Option<&'static [u8]> {
    TOOLS.iter().find(|(typed, _)| *typed == name).map(|(_, service)| *service)
}

pub fn is_tool(name: &[u8]) -> bool {
    service_for(name).is_some()
}

/// Spawn the baked tool named by `args[0]` with the rest as its argv, and return
/// a drain job that streams its stdout to the terminal. `None` on a spawn error,
/// with the reason already pushed to the scrollback.
pub fn prepare(state: &mut State, args: &[&[u8]]) -> Option<InstallJob> {
    let name = args[0];
    // `is_tool` gated this call, so the lookup cannot miss. Falling back to the
    // typed name rather than unwrapping keeps a future caller that skips the
    // gate on the ordinary "no such service" path instead of a panic.
    let handle = service_for(name).unwrap_or(name);
    let mut service = Vec::with_capacity(5 + handle.len());
    service.extend_from_slice(b"tool.");
    service.extend_from_slice(handle);

    let argv = argv_blob(args);
    let rc = mk_tool_run(&service, &argv);
    if rc < 0 {
        super::tool_refused::refused(state, name, rc);
        return None;
    }
    Some(InstallJob::new(rc as u32))
}

// argv as the tool sees it: argv[0] is the command name, then each argument,
// NUL-separated, which is what the kernel splits into the process argv.
fn argv_blob(args: &[&[u8]]) -> Vec<u8> {
    let mut blob = Vec::new();
    for (i, a) in args.iter().enumerate() {
        if i > 0 {
            blob.push(0);
        }
        blob.extend_from_slice(a);
    }
    blob
}
