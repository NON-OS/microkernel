// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/* A launch that opened nothing, said on screen and in the log. */

use nonos_app_skeleton::log_line::{say as log, Line};

use super::is_off;
use super::say::{label, say};
use crate::server::handlers::launcher_request;
use crate::state::says::{not_opened, NOTHING_REFUSED, NOT_ASKED, OFF_AT_SETUP};
use crate::state::toast::UptimeMs;
use crate::state::{Context, NotifyLevel};

/*
 * Say why a launch opened nothing, on screen and in the log (`log LAUNCH`):
 * off at setup, Qwen's own reason, or the kernel's answer to the spawn it
 * refused with no window of the app left to raise.
 */
pub fn toast_failed(ctx: &mut Context, service: &[u8], now: UptimeMs) {
    let refusal = launcher_request::take_refusal();
    if is_off(service) {
        say(ctx, service, OFF_AT_SETUP, NotifyLevel::Warn, now);
        return;
    }
    if service == super::qwen::SERVICE {
        qwen_failed(ctx, now);
        return;
    }
    let tail = not_opened(refusal);
    say(ctx, service, tail, NotifyLevel::Error, now);
    let line = Line::new(b"LAUNCH").text(label(service)).text(tail);
    let line = match refusal {
        NOT_ASKED => line.text(b" (one window only, and it is not running)"),
        NOTHING_REFUSED => line.text(b" (no spawn asked; its window's process is gone)"),
        rc => line.text(b" (kernel spawn answer ").num(rc).text(b"; no window of it to raise)"),
    };
    let _ = log(&line);
}

/* Qwen opened nothing: its reason, which the Terminal's qwen also gives. */
pub(super) fn qwen_failed(ctx: &mut Context, now: UptimeMs) {
    let why = super::qwen::why();
    ctx.toasts.push(why, NotifyLevel::Warn, now);
    let _ = log(&Line::new(b"LAUNCH")
        .text(why)
        .text(b" (run answer ")
        .num(super::qwen::last())
        .text(b")"));
}
