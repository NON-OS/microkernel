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

use nonos_app_skeleton::log_line::{say as log, Line};

use super::is_off;
use crate::server::handlers::launcher_request::{self, LaunchOutcome};
use crate::state::apps::LauncherApp;
use crate::state::says::{named, not_opened, NOTHING_REFUSED, NOT_ASKED, OFF_AT_SETUP};
use crate::state::toast::{UptimeMs, TOAST_TEXT_MAX};
use crate::state::{expect_window, Context, NotifyLevel, Uptime, LAUNCHER_APPS};

/* A dock launch, never asked of the kernel for an app it will not spawn. */
pub fn request(app: &LauncherApp) -> LaunchOutcome {
    if is_off(app.service) {
        return LaunchOutcome::Failed;
    }
    if app.service == super::qwen::SERVICE {
        return super::qwen::open();
    }
    launcher_request::request(app)
}

/*
 * Launch `service` from a menu, the Launchpad, a dialog or the escape chord,
 * or say why nothing opened: off at setup, Qwen's own reason, or that the
 * kernel neither spawned a window nor had one to raise.
 */
pub fn open(ctx: &mut Context, service: &[u8]) {
    if is_off(service) {
        say(ctx, service, OFF_AT_SETUP, NotifyLevel::Warn, crate::server::toast_clock::now());
        return;
    }
    if service == super::qwen::SERVICE {
        if super::qwen::open() == LaunchOutcome::Failed {
            qwen_failed(ctx, crate::server::toast_clock::now());
        } else {
            say_default(ctx, crate::server::toast_clock::now());
        }
        return;
    }
    let outcome = launcher_request::request_service(service);
    if outcome == LaunchOutcome::Failed {
        toast_failed(ctx, service, crate::server::toast_clock::now());
    }
    expect(ctx, service, outcome, crate::server::dock_clock::now());
}

/*
 * Follow a launch that should bring a new window: a spawn init accepted, or
 * an instance woken that has no window yet. Past the wait the shell says the
 * app did not open (server/runner/run.rs). A focus that raised a window the
 * app already had brings none, and Qwen's window is the Linux personality's,
 * never matched to the tile, so neither is followed.
 */
pub fn expect(ctx: &mut Context, service: &[u8], outcome: LaunchOutcome, now: Uptime) {
    if service == super::qwen::SERVICE {
        return;
    }
    let Some(index) = LAUNCHER_APPS.iter().position(|a| a.service == service) else { return };
    let has_window = ctx.taskbar.open.get(index).copied().unwrap_or(false);
    let wants = match outcome {
        LaunchOutcome::Queued => true,
        LaunchOutcome::Focused => !has_window,
        LaunchOutcome::Failed => false,
    };
    if wants {
        expect_window(&mut ctx.taskbar, index, now);
    }
}

/* A Qwen window opened on the default tier, not one chosen: said, never silent. */
pub fn say_default(ctx: &mut Context, now: UptimeMs) {
    if let Some(line) = super::qwen::default_said() {
        ctx.toasts.push(&line, NotifyLevel::Info, now);
    }
}

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
fn qwen_failed(ctx: &mut Context, now: UptimeMs) {
    let why = super::qwen::why();
    ctx.toasts.push(why, NotifyLevel::Warn, now);
    let _ = log(&Line::new(b"LAUNCH")
        .text(why)
        .text(b" (run answer ")
        .num(super::qwen::last())
        .text(b")"));
}

/* The app as its tile names it. */
fn label(service: &[u8]) -> &'static [u8] {
    LAUNCHER_APPS.iter().find(|a| a.service == service).map_or(&b"This app"[..], |a| a.label)
}

/* "<App> turned off at setup" or "<App> did not open: why", the app named
 * as its tile names it. */
fn say(ctx: &mut Context, service: &[u8], tail: &[u8], level: NotifyLevel, now: UptimeMs) {
    let mut line = [0u8; TOAST_TEXT_MAX];
    let n = named(label(service), tail, &mut line);
    ctx.toasts.push(&line[..n], level, now);
}
