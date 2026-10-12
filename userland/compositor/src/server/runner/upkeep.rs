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

//! The display's upkeep between frames: on GOP, one name of the display
//! list asked per tick on Backoff's schedule; on a wire driver, the poll
//! every 500 ms, or at once after a FLUSH found the primary stale. The
//! legacy ops have neither: that driver is driven as it always was.

use crate::display::poll_rule::POLL_MS;
use crate::display::Backoff;
use crate::say::{say, say_display};
use crate::state::Context;

pub struct Upkeep {
    backoff: Backoff,
    last_poll_ms: i64,
}

impl Upkeep {
    pub fn new(now_ms: i64) -> Self {
        Upkeep { backoff: Backoff::new(now_ms), last_poll_ms: now_ms }
    }

    pub fn run(&mut self, ctx: &mut Context, now_ms: i64) {
        if ctx.gop_mode {
            if core::mem::take(&mut ctx.gop_held) {
                self.backoff.held(now_ms);
            }
            if crate::setup::upgrade_from_gop(ctx, &mut self.backoff, now_ms) {
                say_display(ctx, "moved from GOP to");
            }
            return;
        }
        let Some(link) = ctx.wire else { return };
        if link.stale || now_ms.saturating_sub(self.last_poll_ms) >= POLL_MS {
            self.last_poll_ms = now_ms;
            if let Some(what) = crate::setup::poll(ctx) {
                say(&alloc::format!("{}: {}", link.name, what));
                say_display(ctx, "now on");
            }
        }
    }
}
