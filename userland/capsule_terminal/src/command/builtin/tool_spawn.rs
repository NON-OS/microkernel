// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A tool spawned on this screen: its service, its argv, and a retry while it is ending.

use alloc::vec::Vec;
use nonos_libc::{mk_idle_ms, mk_tool_run, Deadline};

use super::super::tool_busy::{may_be_ending, GAP_MS, RETRY_MS};
use super::service_for;

use crate::jobs::box_hold::BoxHold;
use crate::term::state::State;

/// Spawn the baked tool named by `args[0]` with the rest as its argv, its
/// `streams` (`jobs::tty::streams`) on this screen, and return its pid.
/// `None` on a spawn error, with the reason already pushed to the scrollback;
/// a `linux --box` run's report holder with its pid.
pub fn prepare(state: &mut State, args: &[&[u8]], streams: u64) -> Option<(u32, Option<BoxHold>)> {
    let name = args[0];
    // `is_tool` gated this call, so the lookup cannot miss. Falling back to the
    // typed name rather than unwrapping keeps a future caller that skips the
    // gate on the ordinary "no such service" path instead of a panic.
    let handle = service_for(name).unwrap_or(name);
    let mut service = Vec::with_capacity(5 + handle.len());
    service.extend_from_slice(b"tool.");
    service.extend_from_slice(handle);

    let key = match super::tool_box::boxed(args) {
        Ok(key) => key,
        Err(why) => {
            state.scrollback.push_error(why);
            return None;
        }
    };
    let argv = super::tool_argv::argv(args, key.as_ref());
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
    Some((rc as u32, key.map(BoxHold::new)))
}
