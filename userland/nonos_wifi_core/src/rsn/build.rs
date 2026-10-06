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

//! The RSN element the station sends for a selection: in the association
//! request, and repeated byte for byte in four-way message 2 (IEEE Std
//! 802.11-2020, 12.7.6.3). One CCMP-128 group and pairwise cipher, the one
//! chosen AKM, and the RSN capabilities carrying MFPC/MFPR when management
//! frame protection is negotiated. The group management cipher is left to its
//! BIP-CMAC-128 default, so it is not written. For WPA2-PSK without protection
//! this is exactly `wpa::RSN_IE`, which the proofs pin.

use super::select::{Pmf, Selection};
use super::suite::{CAP_MFPC, CAP_MFPR, OUI_IEEE};

/// The element's total length (id, length and a 20-byte body).
pub const RSNE_LEN: usize = 22;

/// The RSN element id.
pub const EID_RSN: u8 = 48;

/// Build the station's RSN element for `sel`.
pub fn station_rsne(sel: &Selection) -> [u8; RSNE_LEN] {
    let caps = match sel.pmf {
        Pmf::Off => 0,
        Pmf::Capable => CAP_MFPC,
        Pmf::Required => CAP_MFPC | CAP_MFPR,
    };
    let mut e = [0u8; RSNE_LEN];
    e[0] = EID_RSN;
    e[1] = (RSNE_LEN - 2) as u8;
    e[2..4].copy_from_slice(&1u16.to_le_bytes()); // version
    e[4..7].copy_from_slice(&OUI_IEEE);
    e[7] = 4; // group cipher CCMP-128
    e[8..10].copy_from_slice(&1u16.to_le_bytes());
    e[10..13].copy_from_slice(&OUI_IEEE);
    e[13] = 4; // pairwise cipher CCMP-128
    e[14..16].copy_from_slice(&1u16.to_le_bytes());
    e[16..19].copy_from_slice(&OUI_IEEE);
    e[19] = sel.akm.suite_type();
    e[20..22].copy_from_slice(&caps.to_le_bytes());
    e
}
