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

//! Telling the compositor a new picture is in the surface.
//!
//! A commit whose answer did not come in time still reached the compositor
//! (the kernel queued it when it was sent) and the compositor repaints from
//! it when it gets to it, so that is done; asking again would only make it
//! compose the whole screen twice. A commit the kernel did not deliver, or
//! the compositor refused, is asked again at a growing interval. Until the
//! compositor has the surface in its scene there is nothing to commit: the
//! submit that puts it there repaints all of it.

use crate::compositor_client::push_damage_commit;
use crate::compositor_client::wire::TIMED_OUT;
use crate::state::Context;

/// A new picture is in the surface: commit it now.
pub fn request_commit(ctx: &mut Context) {
    ctx.commit_pending = true;
    ctx.commit_backoff.reset();
    keep_committed(ctx);
}

/// Each turn of the service loop: commit a picture not yet committed, when
/// the backoff says so.
pub fn keep_committed(ctx: &mut Context) {
    if !ctx.commit_pending {
        return;
    }
    if !ctx.registered {
        ctx.commit_pending = false;
        return;
    }
    if !ctx.commit_backoff.due() {
        return;
    }
    let rid = ctx.issue_request_id();
    match push_damage_commit(ctx.compositor_port, rid, 0, 0, ctx.width, ctx.height) {
        Err(e) if e != TIMED_OUT => ctx.commit_backoff.failed(),
        _ => ctx.commit_pending = false,
    }
}
