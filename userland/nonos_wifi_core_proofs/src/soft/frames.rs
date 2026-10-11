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

//! The frames the simulated access point sends: beacons and probe responses
//! naming its network, its authentication and association answers, EAPOL in
//! data frames, and a deauthentication.

use crate::ap_sim::AA;

pub const RSNE_PSK: [u8; 22] =
    [0x30, 0x14, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 2, 0, 0];
const RATES: [u8; 10] = [1, 8, 0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24];

pub fn mgmt(subtype: u8, dst: [u8; 6], body: &[u8]) -> Vec<u8> {
    let mut f = vec![subtype << 4, 0, 0, 0];
    f.extend_from_slice(&dst);
    f.extend_from_slice(&AA);
    f.extend_from_slice(&AA);
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(body);
    f
}

/// A beacon (`to` broadcast) or probe response from the AP on `channel`.
pub fn beacon(subtype: u8, to: [u8; 6], ssid: &[u8], channel: u8, rsne: &[u8]) -> Vec<u8> {
    let mut body = vec![0u8; 8];
    body.extend_from_slice(&[0x64, 0, 0x11, 0x04, 0, ssid.len() as u8]);
    body.extend_from_slice(ssid);
    body.extend_from_slice(&RATES);
    body.extend_from_slice(&[3, 1, channel]);
    body.extend_from_slice(rsne);
    mgmt(subtype, to, &body)
}

/// A data frame from the AP to `dst` carrying `llc` (from the LLC/SNAP
/// header on), in the clear.
pub fn from_ds(dst: [u8; 6], llc: &[u8]) -> Vec<u8> {
    let mut f = vec![0x08, 0x02, 0, 0];
    f.extend_from_slice(&dst);
    f.extend_from_slice(&AA);
    f.extend_from_slice(&AA);
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(llc);
    f
}

pub fn eapol_to(dst: [u8; 6], eapol: &[u8]) -> Vec<u8> {
    let mut llc = vec![0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e];
    llc.extend_from_slice(eapol);
    from_ds(dst, &llc)
}

/// The EAPOL payload of a data frame, if it carries one.
pub fn eapol_of(f: &[u8]) -> Option<&[u8]> {
    let p = (24..f.len().checked_sub(8)?)
        .find(|&p| f[p..p + 8] == [0xaa, 0xaa, 3, 0, 0, 0, 0x88, 0x8e])?;
    Some(&f[p + 8..])
}
