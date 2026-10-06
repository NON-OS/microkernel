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

use crate::compositor_client::push_damage_commit;
use crate::state::Context;

const COMMIT_TRIES: u32 = 4;

/// Ask for the first full frame. Best effort: the runner's first repaint
/// commits the whole screen again, so a compositor too busy to answer here is
/// no reason to start setup over.
pub fn commit_overlay(ctx: &mut Context) {
    let (port, w, h) = (ctx.compositor_port, ctx.width, ctx.height);
    let mut rid = ctx.issue_request_id();
    let _ = crate::setup::prime::patient::patiently(COMMIT_TRIES, 20, || {
        rid = rid.wrapping_add(1);
        push_damage_commit(port, rid, 0, 0, w, h)
    });
}
