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

//! Build the plaintext 802.11 data MPDU for an ethernet frame: the ToDS MAC
//! header with the Protected bit clear, the LLC/SNAP shim, the ethertype
//! and the payload.

use super::*;

/// Build the plaintext 802.11 data MPDU for an ethernet frame: the ToDS MAC
/// header with the Protected bit clear, the LLC/SNAP shim, the ethertype and
/// the payload. `our_mac` is the station, `bssid` the AP. This is the frame a
/// hardware-CCMP chip is handed directly (it inserts the CCMP header and
/// encrypts from its key cache); a software-CCMP chip passes it to `protect`.
/// Returns `None` if the ethernet frame is malformed.
pub fn build_data(eth: &[u8], our_mac: MacAddr, bssid: MacAddr, seq: u16) -> Option<Vec<u8>> {
    if eth.len() < ETH_HEADER_LEN {
        return None;
    }
    let mut dst = [0u8; 6];
    dst.copy_from_slice(&eth[0..6]);
    let fc = frame_control(TYPE_DATA, 0) | FC_TO_DS;
    let sc = seq_control(seq);
    // ToDS addressing: addr1 = BSSID, addr2 = source (us), addr3 = destination.
    let mut mpdu = Vec::with_capacity(MAC_HEADER_LEN + 8 + eth.len() - ETH_HEADER_LEN);
    mpdu.extend_from_slice(&fc.to_le_bytes());
    mpdu.extend_from_slice(&[0, 0]); // duration
    mpdu.extend_from_slice(&bssid);
    mpdu.extend_from_slice(&our_mac);
    mpdu.extend_from_slice(&dst);
    mpdu.extend_from_slice(&sc.to_le_bytes());
    // Plaintext body: LLC/SNAP, the ethertype, then the ethernet payload.
    mpdu.extend_from_slice(&LLC_SNAP);
    mpdu.extend_from_slice(&eth[12..14]);
    mpdu.extend_from_slice(&eth[ETH_HEADER_LEN..]);
    Some(mpdu)
}
