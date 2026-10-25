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

//! Ctrl+Alt+Esc, the chord the input router hands only to the shell. It brings
//! Process Manager forward over whatever window has the screen, so a window
//! that hangs or swallows every key can always be found and ended.

use crate::server::handlers::launcher_request::{self, LaunchOutcome};
use crate::server::repaint::repaint;
use crate::state::{Context, LAUNCHER_APPS};

/// The router's one rule for the chord, shared rather than copied.
#[path = "../../../../capsule_input_router/src/route/chord.rs"]
mod chord;

pub use crate::state::trusted_rule::opens_terminal;
pub use chord::{is_reserved_chord, is_secure_attention, is_terminal_chord};

const PROCESS_MANAGER: &[u8] = b"app.process_manager";
const TERMINAL: &[u8] = b"app.terminal";

pub fn bring_process_manager(ctx: &mut Context) {
    bring(ctx, PROCESS_MANAGER);
}

/// Ctrl+Alt+T: the Terminal forward, or opened when it is not running.
pub fn bring_terminal(ctx: &mut Context) {
    bring(ctx, TERMINAL);
}

fn bring(ctx: &mut Context, service: &[u8]) {
    clear_chrome(ctx);
    let running = LAUNCHER_APPS
        .iter()
        .position(|a| a.service == service)
        .map(|index| launcher_request::focus_app(ctx, index));
    if running != Some(LaunchOutcome::Focused) {
        crate::apps_off::open(ctx, service);
    }
}

/// Close the Launchpad and the menus, and repaint: nothing of the shell's may
/// cover what a chord brings up, and they draw in the chrome band, over every
/// window.
pub fn clear_chrome(ctx: &mut Context) {
    if ctx.launchpad_ui.launchpad {
        super::launchpad::close(ctx);
    }
    ctx.desk_ui.desktop_menu = None;
    ctx.desk_ui.menu_hover = None;
    ctx.desk_ui.menu_target = None;
    ctx.menubar.open = None;
    ctx.menubar.hover = None;
    repaint(ctx);
}
