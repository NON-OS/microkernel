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

//! Convert a plaintext 802.11 data MPDU (MAC header, LLC/SNAP, ethertype,
//! payload) into an ethernet frame, handling both the AP-to-station
//! (FromDS) and station-to-AP (ToDS) address layouts.

use super::*;

/// Convert a plaintext 802.11 data MPDU (MAC header, LLC/SNAP, ethertype,
/// payload) into an ethernet frame, handling both the AP-to-station (FromDS)
/// and station-to-AP (ToDS) address layouts. This is the receive path after
/// decryption, whether by software `unprotect` or a hardware-CCMP chip that
/// delivered the frame plaintext. Returns `None` if the MPDU is too short or
/// its body is not LLC/SNAP encapsulated.
pub fn parse_data(mpdu: &[u8]) -> Option<Vec<u8>> {
    if mpdu.len() < MAC_HEADER_LEN {
        return None;
    }
    let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]);
    // A QoS data frame carries a longer header; size the body from the frame so
    // the LLC/SNAP shim is found whether or not the AP used QoS.
    let hlen = header_len(fc);
    // A hardware-CCMP chip delivers a decrypted frame with the 802.11 header and
    // the 8-byte CCMP header (IV and packet number) still in place, and leaves the
    // Protected bit set in the frame control. The plaintext body begins after both
    // headers. Skipping the CCMP header when protected is what puts the LLC/SNAP
    // shim at the expected offset; without it every hardware-decrypted frame, the
    // DHCP reply included, failed the LLC/SNAP check and was silently dropped.
    let ccmp = if fc & FC_PROTECTED != 0 { CCMP_HDR_LEN } else { 0 };
    if mpdu.len() < hlen + ccmp + 8 {
        return None;
    }
    let mut a1 = [0u8; 6];
    let mut a2 = [0u8; 6];
    let mut a3 = [0u8; 6];
    a1.copy_from_slice(&mpdu[4..10]);
    a2.copy_from_slice(&mpdu[10..16]);
    a3.copy_from_slice(&mpdu[16..22]);
    let body = &mpdu[hlen + ccmp..];
    if body.len() < 8 || body[0..6] != LLC_SNAP {
        return None;
    }
    // Destination and source depend on the DS bits: FromDS addresses the
    // station as addr1 with the source in addr3; ToDS carries the destination
    // in addr3 and the source in addr2.
    let (da, sa) = if fc & FC_FROM_DS != 0 { (a1, a3) } else { (a3, a2) };
    let mut eth = Vec::with_capacity(ETH_HEADER_LEN + body.len() - 8);
    eth.extend_from_slice(&da);
    eth.extend_from_slice(&sa);
    eth.extend_from_slice(&body[6..8]); // ethertype
    eth.extend_from_slice(&body[8..]); // payload
    Some(eth)
}

/// Decrypt an 802.11 data frame back into an ethernet frame, the software CCMP
/// round trip. Equivalent to `unprotect` followed by `parse_data`; a
/// hardware-CCMP chip uses `parse_data` alone. Returns `None` if the frame is
/// too short, the MIC fails, or the body is not LLC/SNAP encapsulated.
pub fn decap(frame: &[u8], tk: &[u8; 16]) -> Option<Vec<u8>> {
    parse_data(&unprotect(frame, tk)?)
}
