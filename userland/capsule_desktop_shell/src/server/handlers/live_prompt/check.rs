// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ask the policy store, once a tick until it answers, whether this session
//! keeps anything, and bring the prompt up when it does not.

use nonos_policy_client::{get_bool, lookup};
use nonos_policy_proto::Field;

use crate::server::repaint::repaint;
use crate::state::live_prompt::LivePrompt;
use crate::state::Context;

pub fn check(ctx: &mut Context) {
    if !matches!(ctx.live_prompt, LivePrompt::Asking(_)) {
        return;
    }
    let persistent = lookup().and_then(|port| get_bool(port, Field::Persistent));
    ctx.live_prompt = ctx.live_prompt.step(persistent);
    if ctx.live_prompt.showing() {
        let line = b"[SHELL] live session: offering the installer\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
        repaint(ctx);
    }
}
