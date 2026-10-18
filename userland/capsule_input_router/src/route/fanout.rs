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

//! Any other kind of event (touch gestures aside, nothing yet) goes to every
//! process subscribed to it; one that cannot take it is forgotten.

use nonos_libc::InputEvent;

use crate::state::Context;

use super::deliver::deliver_one;

pub fn fan_out(ctx: &mut Context, event: &InputEvent) -> u32 {
    let mut total = 0u32;
    let mut failed = [0u32; crate::state::subscriptions::MAX_SUBSCRIBERS];
    let mut failed_count = 0usize;
    for pid in ctx.subscriptions.match_kind(event.kind) {
        let delivered = deliver_one(pid, event);
        if delivered == 0 && failed_count < failed.len() {
            failed[failed_count] = pid;
            failed_count += 1;
        }
        total += delivered;
    }
    for pid in failed.iter().take(failed_count) {
        ctx.forget_pid(*pid);
    }
    ctx.record(total);
    total
}
