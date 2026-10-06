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

use crate::clients::compositor;
use crate::state::Context;

// How often the display size is asked again once it is known. The compositor
// moves from the firmware framebuffer to the virtio-gpu display once that
// driver answers, at that display's size; asked once, the cursor kept the old
// edges for the rest of the session.
const RECHECK_MS: i64 = 2000;

pub(super) fn refresh_display(ctx: &mut Context) {
    let now = nonos_libc::mk_uptime_ms();
    let since = now.saturating_sub(ctx.display_checked_ms);
    if ctx.cursor.configured && (0..RECHECK_MS).contains(&since) {
        return;
    }
    ctx.display_checked_ms = now;
    let rid = ctx.issue_request_id();
    if let Some((width, height)) = compositor::display_size(&mut ctx.compositor_port, rid) {
        if ctx.cursor.configured {
            ctx.cursor.resize(width, height);
        } else {
            ctx.cursor.configure(width, height);
        }
    }
}
