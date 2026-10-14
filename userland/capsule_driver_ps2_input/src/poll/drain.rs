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

use super::absorb::absorb;
use super::counters::Counters;
use super::drainer::Drainer;
use super::read_port::read_port;
use crate::constants::{DATA_OFFSET, STATUS_OFFSET};
use crate::constants::{STATUS_AUX_DATA, STATUS_OUTPUT_FULL, STATUS_PARITY, STATUS_TIMEOUT};
use crate::mouse::{MouseParser, MouseRing};

const MAX_BYTES_PER_DRAIN: u32 = 16;

/// Empty the controller's output buffer, at most MAX_BYTES_PER_DRAIN bytes,
/// routing each by its AUXDATA bit: keyboard bytes are decoded and posted,
/// aux bytes assembled into mouse packets.
pub fn drain(
    grant_id: u64,
    drainer: &mut Drainer,
    counters: &mut Counters,
    mouse: &mut MouseParser,
    mouse_ring: &mut MouseRing,
    aux_enabled: bool,
) {
    // The packet assembler's clock, read once per drain: the bytes of one
    // drain arrived back to back.
    let mut now_ms = None;
    for _ in 0..MAX_BYTES_PER_DRAIN {
        let Some(status) = read_port(grant_id, STATUS_OFFSET) else { return };
        if status & STATUS_OUTPUT_FULL == 0 {
            return;
        }
        if status & STATUS_PARITY != 0 {
            counters.parity_errors = counters.parity_errors.wrapping_add(1);
        }
        if status & STATUS_TIMEOUT != 0 {
            counters.timeout_errors = counters.timeout_errors.wrapping_add(1);
        }
        let Some(byte) = read_port(grant_id, DATA_OFFSET) else { return };
        if status & STATUS_AUX_DATA == 0 {
            absorb(drainer, counters, byte);
        } else if aux_enabled {
            // When mouse bring-up failed the aux stream is command echoes and
            // EC garbage, not packets: 0xFA ACKs pass the parser's weak sync
            // check (bit 3 set) and decode into phantom button presses. The
            // bytes are still read, to clear the output buffer.
            let now = *now_ms.get_or_insert_with(|| mk_uptime_ms().max(0) as u64);
            mouse.absorb(byte, now, mouse_ring);
        }
    }
}
