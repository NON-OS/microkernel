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
use crate::state::visible_drop::missed;
use crate::state::Context;

// How often a layer that has not attached is tried again when nothing else
// composes. Everything the compositor changes damages what it changed (a
// layer placed, moved, raised or removed, the cursor, the looks, the band,
// a new canvas or a lost output), so an idle desktop composes and presents
// nothing, where this was a full-screen recomposite and present every four
// seconds even at rest. A layer whose surface did not attach is the one
// thing no damage brings back: its rectangle is damaged here, and drawn
// whole once it attaches (state/visible.rs). On the uptime clock, so a
// pointer posting events every few milliseconds does not bring it sooner.
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
            heal_missed(&mut ctx);
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

fn heal_missed(ctx: &mut Context) {
    missed(&ctx.scene).for_each(|r| ctx.damage.accumulate(r));
}
