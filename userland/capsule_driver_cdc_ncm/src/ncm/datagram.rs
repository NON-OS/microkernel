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

//! The largest datagram, told to a device that offers to take it (NCM 1.0,
//! 6.2.9 and 6.2.10, as recalled, unverified).

use nonos_usbnet::{Bus, Setup};

use super::function::NcmFunction;
use super::requests::{CAP_MAX_DATAGRAM_SIZE, GET_MAX_DATAGRAM_SIZE, SET_MAX_DATAGRAM_SIZE};

/// The datagram size asked for: at least a full Ethernet frame, and no
/// more than a jumbo frame the stack could still take whole.
const DGRAM_MIN: u16 = 1514;
const DGRAM_MAX: u16 = 8192;

/// Only for a device that offers it. The size is read first and written
/// only when it differs, so a device already right is not disturbed; a
/// failure changes nothing.
pub(super) fn max_datagram<B: Bus>(bus: &mut B, f: &NcmFunction) {
    if f.caps & CAP_MAX_DATAGRAM_SIZE == 0 {
        return;
    }
    let want = f.max_segment.clamp(DGRAM_MIN, DGRAM_MAX);
    let mut now = [0u8; 2];
    let get = Setup::class(true, GET_MAX_DATAGRAM_SIZE, 0, f.comm);
    if bus.control_in(get, &mut now) != Ok(2) || u16::from_le_bytes(now) == want {
        return;
    }
    let set = Setup::class(false, SET_MAX_DATAGRAM_SIZE, 0, f.comm);
    let _ = bus.control_out(set, &want.to_le_bytes());
}
