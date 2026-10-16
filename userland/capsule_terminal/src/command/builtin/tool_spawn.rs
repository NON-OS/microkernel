// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A tool spawned on this screen: its service, its argv, and a retry while it is ending.

use alloc::vec::Vec;
use nonos_libc::{mk_idle_ms, mk_tool_run, Deadline};

use super::super::tool_busy::{may_be_ending, GAP_MS, RETRY_MS};
use super::service_for;

use crate::term::state::State;

/// Spawn the baked tool named by `args[0]` with the rest as its argv, its
/// `streams` (`jobs::tty::streams`) on this screen, and return its pid.
/// `None` on a spawn error, with the reason already pushed to the scrollback.
pub fn prepare(state: &mut State, args: &[&[u8]], streams: u64) -> Option<u32> {
    let name = args[0];
    // `is_tool` gated this call, so the lookup cannot miss. Falling back to the
    // typed name rather than unwrapping keeps a future caller that skips the
    // gate on the ordinary "no such service" path instead of a panic.
    let handle = service_for(name).unwrap_or(name);
    let mut service = Vec::with_capacity(5 + handle.len());
    service.extend_from_slice(b"tool.");
    service.extend_from_slice(handle);

    let argv = argv_blob(args);
    let mut rc = mk_tool_run(&service, &argv);
    let until = Deadline::after_ms(RETRY_MS);
    while may_be_ending(name, rc) && !until.expired() {
        let _ = mk_idle_ms(GAP_MS);
        rc = mk_tool_run(&service, &argv);
    }
    if rc < 0 {
        super::super::tool_refused::refused(state, name, rc);
        return None;
    }
    crate::jobs::tty::attach_streams(state, rc as u32, streams);
    Some(rc as u32)
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
