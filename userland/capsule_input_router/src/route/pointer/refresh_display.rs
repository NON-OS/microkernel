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

use nonos_libc::InputEvent;

use crate::clients::compositor;
use crate::state::Context;

// How often the display size is asked again once it is known: the fallback
// for a size notice lost or sent before the router was up (cursor_edges.rs).
// Until it is known, every pointer event asks.
const RECHECK_MS: i64 = 2000;

/// True once the cursor's edges are the display's. Until then `event`'s
/// absolute position, if it has one, is held for when they are.
pub(super) fn refresh_display(ctx: &mut Context, event: &InputEvent) -> bool {
    let now = nonos_libc::mk_uptime_ms();
    let since = now.saturating_sub(ctx.display_checked_ms);
    if ctx.cursor.configured && (0..RECHECK_MS).contains(&since) {
        return true;
    }
    ctx.display_checked_ms = now;
    let rid = ctx.issue_request_id();
    if let Some((width, height)) = compositor::display_size(&mut ctx.compositor_port, rid) {
        ctx.cursor.adopt(width, height);
    }
    if !ctx.cursor.configured {
        ctx.cursor.hold(event);
    }
    ctx.cursor.configured
}
