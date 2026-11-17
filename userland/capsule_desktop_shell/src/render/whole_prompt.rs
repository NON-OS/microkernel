// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A prompt whose body the user must read whole before acting on it, such
//! as the capability list an install is approved against. It asks the
//! shared `Dialog` for every line the body wraps to, so only the screen it
//! is drawn on can cut it, and when the screen still does, a warning note
//! says so.

use nonos_toolkit::widgets::{Dialog, Rect, Tone};

/// The most body lines such a prompt asks for: all of them. A fixed count
/// would be a second cut that a longer capability table could outgrow.
pub const WHOLE: u32 = u32::MAX;

/// `d` with its whole body where `at` has room for it, and with `note` in
/// the Warn tone when `at` still cuts the body.
pub fn whole<'a>(d: Dialog<'a>, note: &'a str, at: Rect) -> Dialog<'a> {
    let d = d.body_lines(WHOLE);
    if d.cut(at) {
        d.note(note, Tone::Warn)
    } else {
        d
    }
}

/// `d` as it is drawn and lit: its acting button, the first, disabled while
/// `at` cuts its body, since that button then answers nothing. Clicks are
/// still tested on `d` itself, so a press there is held, not read as a
/// press off every button.
pub fn held<'a>(d: Dialog<'a>, at: Rect) -> Dialog<'a> {
    if d.cut(at) {
        d.disable(0)
    } else {
        d
    }
}
