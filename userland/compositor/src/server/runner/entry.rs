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

use nonos_libc::mk_uptime_ms;

use super::drain::drain_ipc;
use super::upkeep::Upkeep;
use crate::frame_pacer;
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::state::Context;

// Force a full recomposite this often. Damage tracking only repaints the
// rectangles a client reports; a region left stale by a transient (a
// scanout size settling during boot, a client that damaged too little)
// would otherwise never heal. Every four seconds is enough for that rare
// staleness without a full-screen present more than once a second at idle.
// On the uptime clock: counted in loop passes it was every 240 passes, and
// every IPC ends a pass early, so a mouse or a scrolling client that posts
// events every few milliseconds could bring a full-screen recomposite and
// present (on a 4K panel, a 1920x1080 canvas doubled and 33 MB copied)
// many times more often than that, in the middle of the scroll.
const HEAL_INTERVAL_MS: i64 = 4000;

pub fn run(mut ctx: Context) -> ! {
    let mut rx = [0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = [0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut last_heal = mk_uptime_ms();
    let mut upkeep = Upkeep::new(last_heal);
    ctx.first_frame_since = Some(last_heal);
    let mut notice = crate::server::trusted_notice::Notice::new(last_heal);
    loop {
        drain_ipc(&mut ctx, &mut rx, &mut tx);
        let now = mk_uptime_ms();
        upkeep.run(&mut ctx, now);
        notice.run(&ctx, now);
        if now.saturating_sub(last_heal) >= HEAL_INTERVAL_MS {
            last_heal = now;
            ctx.damage.mark_full(ctx.width, ctx.height);
        }
        match frame_pacer::tick(&mut ctx) {
            Ok(()) => {}
            Err(e) if !ctx.scanout_error_reported => {
                // Once: a display that refuses every frame would fill the log.
                crate::say::say(&alloc::format!("present refused: {}", e));
                ctx.scanout_error_reported = true;
            }
            Err(_) => {}
        }
        // No vsync sleep here: the blocking first receive in drain_ipc paces the
        // loop to a frame while still waking the instant a client commits damage.
        // The present itself already happened in tick(); waiting on the periodic
        // vsync tick only reintroduced the unreliable-timer dependency that left
        // updates stuck until an input event.
    }
}
