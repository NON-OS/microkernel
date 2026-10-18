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

//! A pointer frame: a drag's release before anything else, then motion,
//! and a press or touch.

use super::drag::drop_drag;
use crate::protocol::read_i32;
use crate::state::input_source_rule::Routed;
use crate::state::Context;
use nonos_libc::{
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_TOUCH,
};

pub(super) fn pointer(ctx: &mut Context, by: Routed, buf: &[u8], kind: u16) -> bool {
    let Some(x) = read_i32(buf, 16) else {
        return true;
    };
    let Some(y) = read_i32(buf, 20) else {
        return true;
    };
    // The release of a drag must never be missed, or the icon stays glued to
    // the cursor; resolve it before the coordinate bail-out below (the drop
    // uses the last tracked drag position, not this event's).
    if kind == INPUT_KIND_BUTTON_UP && ctx.desk_ui.drag_from.is_some() {
        drop_drag(ctx);
        return true;
    }
    if x < 0 || y < 0 {
        return true;
    }
    if kind == INPUT_KIND_POINTER_ABS {
        return super::motion::motion(ctx, x, y);
    }
    if kind == INPUT_KIND_BUTTON_UP {
        return true;
    }
    if !matches!(kind, INPUT_KIND_TOUCH | INPUT_KIND_BUTTON_DOWN) {
        return true;
    }
    super::press::press(ctx, by, buf, kind, x, y)
}
