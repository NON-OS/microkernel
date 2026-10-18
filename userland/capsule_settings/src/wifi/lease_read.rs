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

//! The DHCP client's lease-status answer read: bound or not, and when
//! bound the address, gateway and DNS server it holds.

use super::lease::{Lease, NetStatus};

// Generous: a bound net_core spends most of a poll draining frames, so a short
// deadline misses its reply and the panel wrongly reads the stack as absent.
const STATE_BOUND: u8 = 3;

// Decode the 22-byte body: state, address fields, then the bound port.
pub(super) fn decode(b: &[u8]) -> NetStatus {
    let bound = u32::from_le_bytes([b[18], b[19], b[20], b[21]]);
    if b[0] != STATE_BOUND {
        return NetStatus::Unbound { port: bound };
    }
    let lease = Lease {
        ip: [b[1], b[2], b[3], b[4]],
        prefix: b[5],
        gw: [b[6], b[7], b[8], b[9]],
        dns: [b[10], b[11], b[12], b[13]],
    };
    NetStatus::Bound { lease, port: bound }
}
