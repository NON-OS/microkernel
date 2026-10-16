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

//! A key press: the reserved chord, the system keys, Ctrl's commands, the
//! dialogs, and an inline rename or the Launchpad's field.

use crate::protocol::{read_u16, read_u32};
use crate::server::desktop;
use crate::state::Context;

/// The Ctrl bit of a key event's modifier flags, as app_skeleton names it.
const MOD_CTRL: u16 = nonos_app_skeleton::MOD_CTRL;

pub(super) fn key(ctx: &mut Context, buf: &[u8], kind: u16) -> bool {
    let code = read_u32(buf, 12).unwrap_or(0);
    let flags = read_u16(buf, 10).unwrap_or(0);
    if super::super::handlers::escape_chord::is_reserved_chord(kind, code, flags) {
        super::super::handlers::escape_chord::bring_process_manager(ctx);
        return true;
    }
    // The volume and power keys come here whatever has focus, with
    // whatever modifiers are held, and are never text.
    if super::super::handlers::system_keys::key(ctx, code) {
        return true;
    }
    // With Ctrl held a key is a command, never text: Ctrl+V pastes, and
    // the rest used to type their letter into the field.
    if flags & MOD_CTRL != 0 {
        if super::super::paste::is_paste_key(code) {
            if ctx.desk_ui.rename.is_some() {
                desktop::rename_paste(ctx);
                super::super::repaint::repaint(ctx);
            } else if ctx.launchpad_ui.launchpad {
                super::super::handlers::launchpad_key::paste(ctx);
            }
        }
        return true;
    }
    if super::super::handlers::dialog_key::key(ctx, code) {
        return true;
    }
    if ctx.desk_ui.rename.is_some() {
        desktop::rename_key(ctx, code);
        super::super::repaint::repaint(ctx);
    } else if ctx.launchpad_ui.launchpad {
        super::super::handlers::launchpad_key::key(ctx, code);
    }
    true
}
