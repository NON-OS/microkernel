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

//! The scan reply, from the background scanner's running picture.

use super::super::radio::Radio;
use super::super::scanner::Scanner;
use super::ops::WIFI_HDR;
use crate::status;

// The scan is answered instantly from the background scanner's running picture.
// The reply carries three little-endian counters before the network list: passes
// taken, raw frames the ring delivered, and beacons parsed. Zero passes means the
// scanner never got the CPU; passes but zero raw means the ring delivered nothing;
// raw but zero beacons means frames arrive but do not parse.
pub(super) fn scan_reply(radio: &Radio, scanner: &Scanner, out: &mut [u8]) -> Option<usize> {
    let n = match radio {
        Radio::Up(_) => {
            out[WIFI_HDR..WIFI_HDR + 4].copy_from_slice(&scanner.steps.to_le_bytes());
            out[WIFI_HDR + 4..WIFI_HDR + 8].copy_from_slice(&scanner.raw.to_le_bytes());
            out[WIFI_HDR + 8..WIFI_HDR + 12].copy_from_slice(&scanner.beacons.to_le_bytes());
            log_scan(scanner);
            12 + scanner.cache().encode(&mut out[WIFI_HDR + 12..])
        }
        Radio::Down => {
            out[WIFI_HDR..WIFI_HDR + 12].fill(0);
            out[WIFI_HDR + 12] = 0;
            13
        }
    };
    Some(WIFI_HDR + n)
}

// The scan pipeline counters as a debug line, silenced by default (the panel
// shows the same numbers). Left in the code for the next hardware bring-up.
fn log_scan(scanner: &Scanner) {
    status::debug(b"[rtl8821ce] scan steps=");
    status::debug_number(scanner.steps);
    status::debug(b" rx=");
    status::debug_number(scanner.raw);
    status::debug(b" beacons=");
    status::debug_number(scanner.beacons);
    status::debug(b" networks=");
    status::debug_number(scanner.cache().count() as u32);
    status::debug(b"\n");
}
