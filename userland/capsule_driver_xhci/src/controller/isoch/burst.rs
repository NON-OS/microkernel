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

//! The burst fields of an isoch TD (xHCI 1.2 section 4.11.2.3).
//! Below SuperSpeed both count packets: TBC is 0 and TLBPC is the TD's
//! packets less one. At SuperSpeed they count bursts of `max_burst + 1`.

/// `(TBC, TLBPC)` for a TD of `len` bytes on an endpoint of `max_packet`.
pub fn burst_fields(len: u32, max_packet: u16, max_burst: u8, superspeed: bool) -> (u8, u8) {
    let packets = len.div_ceil(max_packet.max(1) as u32).max(1);
    if !superspeed {
        return (0, (packets - 1).min(15) as u8);
    }
    let per_burst = max_burst as u32 + 1;
    let tbc = packets.div_ceil(per_burst) - 1;
    let tlbpc = match packets % per_burst {
        0 => max_burst as u32,
        residue => residue - 1,
    };
    (tbc.min(3) as u8, tlbpc as u8)
}
