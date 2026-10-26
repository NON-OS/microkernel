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

use alloc::vec;

use nonos_libc::mk_uptime_ms;

use super::drain::drain;
use super::refresh_clock::refresh_clock;
use super::tick::{tick_due, wake_at};
use crate::protocol::{HDR_LEN, IPC_PAYLOAD_MAX};
use crate::render::sync_toast_layer;
use crate::state::Context;

pub fn run(mut ctx: Context) -> ! {
    crate::server::paint_initial::paint_initial(&mut ctx);
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut last_tick_ms: i64 = mk_uptime_ms();
    refresh_clock(&mut ctx);
    loop {
        let due = super::timed::due(&ctx);
        drain(&mut ctx, &mut rx, &mut tx, wake_at(last_tick_ms, due));
        // A menu, a dialog or the Launchpad opened or closed by what was just
        // handled takes or gives back the pointer here.
        crate::server::grabs::sync(&mut ctx);
        // A full-screen window shown or gone, or the pointer at the bottom
        // edge: the dock drawn or cleared, its window opened or closed.
        crate::server::dock_sync::sync(&mut ctx);
        /*
         * The tick is timed on the uptime clock. The wall clock can step back
         * when NTP corrects it, or read as an error before it is set, and a
         * tick timed on it then stopped for as long as the step: no clock, no
         * toasts expiring, no subscriptions retried.
         */
        let tick_ms = mk_uptime_ms();
        if tick_due(tick_ms, last_tick_ms) {
            super::on_tick::on_tick(&mut ctx, tick_ms);
            last_tick_ms = tick_ms;
        }
        /*
         * The toasts and the switcher's strip, on every pass rather than on
         * the tick: one whose time is up leaves now, the loop having woken for
         * it (`wake_at`), and a toast pushed by any handler is put on screen
         * now, whether or not that handler synced the panel. The panel is synced only when the
         * queue moved, so a pass with nothing new costs one comparison.
         */
        super::timed::expire(&mut ctx);
        if ctx.toast_view.toasts_synced != ctx.toasts.generation() {
            sync_toast_layer(&mut ctx);
        }
        /*
         * No vsync sleep: drain() already blocks on the IPC inbox (waking the
         * instant an input or notify message arrives, and timing out every
         * RECV_BLOCK ms so the clock still ticks). Parking again on the vsync
         * tick only added up to a frame of latency to every click, since an
         * inbound event cannot preempt a vsync wait.
         */
    }
}
