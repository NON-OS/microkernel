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
//! OP_START: the peer asked of the kernel again, the compositor's indicator
//! shown, then the stream begun on the device, or no stream.

use crate::device::Dev;
use crate::indicator::shown_first;
use crate::notify::capture;
use crate::ops::holds;
use crate::ring::Ring;
use crate::start::start;
use crate::stream::stop;
use crate::uac::{RATE_HZ, SAMPLE_BYTES};
use crate::wire::{E_NODEV, E_PERM};

/// A stream started for `peer`: `channels u8, 0, 0, 0, rate u32`, begun
/// only once the compositor shows the indicator (Decision 9: no
/// indicator, no stream).
pub fn begin(d: &mut Dev, peer: u32) -> Result<[u8; 8], i32> {
    d.session.start(peer, holds(peer))?;
    if d.streaming {
        stop(d);
        d.session.start(peer, true)?;
    }
    let (Some(mut bus), Some(b)) = (d.bus(), d.bound.as_ref()) else {
        d.session.stop();
        return Err(E_NODEV);
    };
    let channels = b.nic.s.channels;
    let (begun, hide) = (|| start(&mut bus, &b.nic), || _ = capture(false));
    if let Err(e) = shown_first(E_PERM, || capture(true), begun, hide) {
        d.session.stop();
        return Err(e);
    }
    let frame = SAMPLE_BYTES as usize * channels as usize;
    let share = b.nic.timing.packet_bytes as usize + frame;
    (d.streaming, d.ring) = (true, Ring::new(frame).sharing(share));
    let r = RATE_HZ.to_le_bytes();
    Ok([channels, 0, 0, 0, r[0], r[1], r[2], r[3]])
}
