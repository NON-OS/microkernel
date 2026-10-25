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

use super::breadcrumb::{step, turn as begin};
use super::drain::drain_ipc;
use super::heal::{heal_missed, HEAL_INTERVAL_MS};
use super::stall_rule::Step;
use super::turn_watch::{Turn, TurnWatch};
use super::upkeep::Upkeep;
use crate::frame_pacer;
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::state::Context;

pub fn run(mut ctx: Context) -> ! {
    let mut rx = [0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = [0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut last_heal = mk_uptime_ms();
    let mut upkeep = Upkeep::new(last_heal);
    ctx.first_frame_since = Some(last_heal);
    let mut notice = crate::server::trusted_notice::Notice::new(last_heal);
    let mut shown = crate::server::display_notice::DisplayNotice::new(last_heal);
    let mut watch = TurnWatch::default();
    super::stall_watch::start();
    loop {
        let t0 = mk_uptime_ms();
        begin(t0);
        let request = drain_ipc(&mut ctx, &mut rx, &mut tx);
        let now = mk_uptime_ms();
        step(Step::Upkeep, now);
        upkeep.run(&mut ctx, now);
        notice.run(&ctx, now);
        shown.run(&ctx, now);
        if now.saturating_sub(last_heal) >= HEAL_INTERVAL_MS {
            last_heal = now;
            heal_missed(&mut ctx);
        }
        let t2 = mk_uptime_ms();
        step(Step::Compose, t2);
        let ticked = frame_pacer::tick(&mut ctx);
        step(Step::Wait, mk_uptime_ms());
        let at = [t0, now, t2, mk_uptime_ms()];
        let turn = Turn { at, request, released: ctx.first_frame_since.is_none() };
        watch.observe(&turn).iter().for_each(|l| crate::say::say(l));
        match ticked {
            Ok(()) => {}
            // Once: a display that refuses every frame would fill the log.
            Err(e) if !ctx.scanout_error_reported => {
                crate::say::say(&alloc::format!("present refused: {}", e));
                ctx.scanout_error_reported = true;
            }
            Err(_) => {}
        }
        // No vsync sleep: drain_ipc's first receive paces the loop (drain.rs).
    }
}
