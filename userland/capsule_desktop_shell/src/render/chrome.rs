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

use super::layout::bottom_dock_rect;
use super::paint_bottom_taskbar;
use crate::state::Context;

mod clear_overlay;
mod constants;
mod frame;

pub use frame::paint_chrome;

/// The chrome band: everything the shell draws over the windows.
pub(super) fn paint_over_windows(ctx: &mut Context) {
    clear_overlay::clear_overlay(ctx);
    super::topbar::paint(ctx);
    // A dragged icon rides under the cursor, over every window, until it is
    // dropped.
    super::desktop_icons::paint_drag_ghost(ctx);
    if ctx.taskbar.visible {
        super::panel::shadow_panel(
            ctx,
            bottom_dock_rect(ctx.width, ctx.height),
            super::palette::R_DOCK,
            super::palette::DOCK,
            super::palette::LINE,
        );
        paint_bottom_taskbar(ctx);
    }
    // The Launchpad, when open, covers the whole desktop and its dock.
    if ctx.launchpad_ui.launchpad {
        super::launchpad::paint_launchpad(ctx);
    }
    // The right-click menu floats above everything else, dock included.
    super::desktop_menu::paint(ctx);
    super::menubar_menu::paint(ctx);
    // The consent modal draws last so it sits above every other layer.
    super::consent::paint_consent(ctx);
    super::pkg_consent::paint_pkg_consent(ctx);
    super::live_prompt::paint_live_prompt(ctx);
    super::delete_prompt::paint_delete_prompt(ctx);
    super::toasts::paint_in_chrome(ctx);
    // Last of all: the system's prompt and the hidden screen cover everything above.
    super::attention::paint(ctx);
}
