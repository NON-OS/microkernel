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

pub use chord::is_reserved_chord;

const PROCESS_MANAGER: &[u8] = b"app.process_manager";

pub fn bring_process_manager(ctx: &mut Context) {
    // Nothing of the shell's may cover it: the Launchpad and the menus draw
    // in the chrome band, over every window.
    if ctx.launchpad {
        super::launchpad::close(ctx);
    }
    ctx.desktop_menu = None;
    ctx.menu_hover = None;
    ctx.menu_target = None;
    ctx.menubar.open = None;
    ctx.menubar.hover = None;
    repaint(ctx);
    let running = LAUNCHER_APPS
        .iter()
        .position(|a| a.service == PROCESS_MANAGER)
        .map(|index| launcher_request::focus_app(ctx, index));
    if running != Some(LaunchOutcome::Focused) {
        crate::apps_off::open(ctx, PROCESS_MANAGER);
    }
}
