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

//! Write a management or data MAC header into `out`, returning the byte
//! count (`MAC_HEADER_LEN`) or `None` if the buffer is too small.

use super::*;

/// Write a management or data MAC header into `out`, returning the byte count
/// (`MAC_HEADER_LEN`) or `None` if the buffer is too small. For the frames the
/// MLME sends: addr1 is the receiver, addr2 the transmitter, addr3 the BSSID.
pub fn write_header(
    out: &mut [u8],
    fc: u16,
    addr1: MacAddr,
    addr2: MacAddr,
    addr3: MacAddr,
    seq: u16,
) -> Option<usize> {
    if out.len() < MAC_HEADER_LEN {
        return None;
    }
    out[0..2].copy_from_slice(&fc.to_le_bytes());
    out[2..4].copy_from_slice(&0u16.to_le_bytes());
    out[4..10].copy_from_slice(&addr1);
    out[10..16].copy_from_slice(&addr2);
    out[16..22].copy_from_slice(&addr3);
    out[22..24].copy_from_slice(&seq_control(seq).to_le_bytes());
    Some(MAC_HEADER_LEN)
}
