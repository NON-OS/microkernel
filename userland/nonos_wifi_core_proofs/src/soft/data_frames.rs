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

//! The data frames the simulated access point sends: plain LLC payloads,
//! and EAPOL in them; and the EAPOL payload of a frame the station sent.

use crate::ap_sim::AA;

/// A data frame from the AP to `dst` carrying `llc` (from the LLC/SNAP
/// header on), in the clear.
pub fn from_ds(dst: [u8; 6], llc: &[u8]) -> Vec<u8> {
    from_ds_of(AA, dst, llc)
}

pub fn from_ds_of(from: [u8; 6], dst: [u8; 6], llc: &[u8]) -> Vec<u8> {
    let mut f = vec![0x08, 0x02, 0, 0];
    f.extend_from_slice(&dst);
    f.extend_from_slice(&from);
    f.extend_from_slice(&from);
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(llc);
    f
}

pub fn eapol_from(from: [u8; 6], dst: [u8; 6], eapol: &[u8]) -> Vec<u8> {
    let mut llc = vec![0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e];
    llc.extend_from_slice(eapol);
    from_ds_of(from, dst, &llc)
}

/// The EAPOL payload of a data frame, if it carries one.
pub fn eapol_of(f: &[u8]) -> Option<&[u8]> {
    let p = (24..f.len().checked_sub(8)?)
        .find(|&p| f[p..p + 8] == [0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e])?;
    Some(&f[p + 8..])
}
