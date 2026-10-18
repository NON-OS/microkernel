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

//! The lock LEDs, brought in line with the lock keys by the serve loop
//! between two waits.

use crate::init::set_leds;
use crate::server::context::Context;
use crate::server::pump::drain_ports;

/// Send the lock LEDs when they differ from what was last sent, once per
/// change: a keyboard that does not acknowledge is not asked again until
/// the next lock key. The first call, at start, sets them whatever the
/// firmware left lit.
pub fn sync_leds(ctx: &mut Context) {
    let want = ctx.drainer.locks;
    if ctx.leds_sent == Some(want) {
        return;
    }
    ctx.leds_sent = Some(want);
    ctx.drainer.reply = None;
    let grant = ctx.driver.pio_grant_id;
    let _ = set_leds(grant, want, || {
        drain_ports(ctx);
        ctx.drainer.reply.take()
    });
}
