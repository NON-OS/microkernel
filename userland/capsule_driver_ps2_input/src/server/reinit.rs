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

//! A mouse that reset (KVM switch, re-plug, EC reset) comes back with
//! reporting off and in the plain 3-byte report, so the serve loop brings
//! it up again, the wheel knock included, while the keyboard is in use: a
//! keyboard byte that lands meanwhile goes through the keyboard's decode
//! like any other. One that does not come back, or that keeps resetting
//! (a failing cable, an EC in a loop), is turned off as at start, and the
//! console says so once.

use nonos_libc::{mk_debug, mk_uptime_ms};

use crate::init::{disable_aux, reenable_mouse, restore_keyboard};
use crate::mouse::MouseParser;
use crate::poll::absorb;
use crate::server::context::Context;

pub fn reinit_mouse(ctx: &mut Context) {
    if !ctx.driver.mouse_enabled || !ctx.mouse.take_reset() {
        return;
    }
    let line: &[u8] = if !ctx.reinits.allow(mk_uptime_ms().max(0) as u64) {
        pointer_off(ctx);
        b"[driver_ps2] the mouse reset itself again after three bring-ups within a minute; pointer off\n"
    } else {
        let (drainer, counters) = (&mut ctx.drainer, &mut ctx.counters);
        match reenable_mouse(ctx.driver.pio_grant_id, &mut |b| absorb(drainer, counters, b)) {
            Ok(wheel) => {
                ctx.driver.mouse_wheel = wheel;
                ctx.mouse = MouseParser::new(wheel);
                b"[driver_ps2] the mouse reset itself and was brought up again\n"
            }
            Err(_) => {
                pointer_off(ctx);
                b"[driver_ps2] the mouse reset itself and did not come back; pointer off\n"
            }
        }
    };
    let _ = mk_debug(line.as_ptr(), line.len());
}

/// The aux port off and the keyboard port on, as when the mouse fails at
/// start; with the pointer off, the drain reads aux bytes only to drop them.
fn pointer_off(ctx: &mut Context) {
    disable_aux(ctx.driver.pio_grant_id);
    restore_keyboard(ctx.driver.pio_grant_id);
    ctx.driver.mouse_enabled = false;
}
