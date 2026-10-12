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
//! The running stream: batches collected from the endpoint into the ring,
//! and the stream stopped, its bandwidth given back and the compositor told.

use crate::batch::payloads;
use crate::device::Dev;
use crate::xhci::{isoch_in, DATA_AT, REPLY_LEN};

/// Batches in a row in error before the stream is stopped.
const LOSSES: u8 = 3;

pub fn pump(d: &mut Dev, resp: &mut [u8; REPLY_LEN]) {
    let (Some(bus), Some(b)) = (d.bus(), d.bound.as_ref()) else { return };
    let Ok(n) = isoch_in(bus.xhci, bus.slot, b.nic.timing.packets, resp) else {
        d.losses += 1;
        if d.losses >= LOSSES {
            stop(d);
        }
        return;
    };
    d.losses = 0;
    for p in payloads(&resp[DATA_AT..DATA_AT + n.min(REPLY_LEN - DATA_AT)]) {
        d.ring.put(p);
    }
}

pub fn stop(d: &mut Dev) {
    if let (Some(mut bus), Some(b)) = (d.bus(), d.bound.as_ref()) {
        let _ = crate::start::stop(&mut bus, &b.nic);
    }
    if core::mem::take(&mut d.streaming) {
        crate::notify::capture(false);
    }
    d.session.stop();
    d.losses = 0;
}
