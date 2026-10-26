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
//! Tell lifecycle subscribers which window has focus whenever it changed
//! (focus/notice.rs), as NOTIFY_KIND_FOCUSED, after each owner hears of its
//! own window (keys_notify.rs).

use crate::focus::model::FocusedRef;
use crate::focus::notice::focus_notice;
use crate::protocol::NOTIFY_KIND_FOCUSED;
use crate::state::Context;

/// Broadcast the focused window when it is not `was`.
pub fn tell_if_moved(ctx: &mut Context, was: Option<FocusedRef>) {
    crate::server::keys_notify::tell_owners(was, ctx.focus.current());
    if let Some((pid, window_id)) = focus_notice(was, ctx.focus.current()) {
        crate::server::notify_fanout::broadcast(ctx, NOTIFY_KIND_FOCUSED, pid, window_id, 0, 0);
    }
}
