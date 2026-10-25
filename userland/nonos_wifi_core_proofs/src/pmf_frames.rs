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

//! The management frames an access point sends a protected station: in the
//! clear, and under the pairwise key, sealed by the AP and handed over either
//! still sealed or as a chip leaves a frame it decrypted.

use crate::ap_sim::AA;
use crate::pmf_sim::tk;
use nonos_wifi_core::dot11::ccmp::encrypt;

/// A management frame of `subtype` from the AP to `dst`, in the clear.
pub fn mgmt(subtype: u8, dst: [u8; 6], body: &[u8]) -> Vec<u8> {
    let mut f = vec![subtype << 4, 0, 0, 0];
    f.extend_from_slice(&dst);
    f.extend_from_slice(&AA);
    f.extend_from_slice(&AA);
    f.extend_from_slice(&[0x40, 0x01]);
    f.extend_from_slice(body);
    f
}

/// `frame` under the pairwise key at `pn`; as the chip hands it over after
/// decrypting it when `hw`: CCMP header kept, plaintext, MIC left in place.
pub fn sealed(frame: &[u8], pn: u64, ptk: &[u8; 48], hw: bool) -> Vec<u8> {
    let enc = encrypt(frame, pn, &tk(ptk)).unwrap();
    if !hw {
        return enc;
    }
    let mut f = enc[..32].to_vec();
    f.extend_from_slice(&frame[24..]);
    f.extend_from_slice(&enc[enc.len() - 8..]);
    f
}
