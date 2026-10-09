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

use crate::focus::hand_off;
use crate::state::Context;

/// After a window closed, was minimised or lost its process: move focus to
/// the window now on top (focus/hand_off.rs) and tell the compositor, which
/// lifts that window's layer as this stack already has it.
///
/// A compositor that does not answer in time changes nothing here. The close
/// or minimise it follows has already happened in the client, which removed
/// its layer, so refusing it (as the handlers did) left this table holding a
/// window the screen no longer showed, and every press over the place it had
/// been went to it.
pub fn hand_off_focus(ctx: &mut Context) {
    if let Some(pid) = hand_off(&ctx.windows, &mut ctx.focus) {
        crate::server::tell_compositor::lift(ctx, pid);
    }
}
