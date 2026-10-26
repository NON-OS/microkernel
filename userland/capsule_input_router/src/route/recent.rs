// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Who the user last pressed or typed into, as the router itself delivered
//! it: a key-down, a button press, a touch or a pad button. Nothing a capsule
//! says about its own input counts, and a capsule cannot post input to
//! another, so the record cannot be forged. The grab rule reads it: the shell
//! may grab over a focused window only while the user's last input went to
//! the shell (a menu it opened on that press), and the next press or key
//! delivered anywhere else ends that.

use nonos_libc::{INPUT_KIND_BUTTON_DOWN, INPUT_KIND_KEY_DOWN, INPUT_KIND_TOUCH};

use super::pad::PAD_BUTTON_DOWN;
use crate::state::Context;

/// An event the user aims at something: a key or button going down, a touch.
pub fn is_press(kind: u16) -> bool {
    matches!(kind, INPUT_KIND_KEY_DOWN | INPUT_KIND_BUTTON_DOWN | INPUT_KIND_TOUCH | PAD_BUTTON_DOWN)
}

/// The record after the router handed `kind` to `pid`: a press that arrived
/// names `pid`; one that did not (no subscription, a full inbox) names
/// nobody, so a press the shell never saw cannot count for it.
pub fn after(last: u32, pid: u32, kind: u16, delivered: u32) -> u32 {
    if !is_press(kind) {
        return last;
    }
    if delivered != 0 {
        pid
    } else {
        0
    }
}

pub fn note(ctx: &mut Context, pid: u32, kind: u16, delivered: u32) {
    ctx.last_input = after(ctx.last_input, pid, kind, delivered);
}
