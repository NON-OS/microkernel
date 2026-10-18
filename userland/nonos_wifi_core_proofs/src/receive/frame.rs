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

//! The access point's side of the receive proofs: data frames from the BSS,
//! plain or under CCMP, and a station associated under the Annex M.6.4 key
//! (`receive::ccmp_tests::tk`).

use crate::ap_sim::{AA, SPA};
use crate::receive::ccmp_tests::tk;
use nonos_wifi_core::ccmp::ccm::ccm_encrypt;
use nonos_wifi_core::dot11::ccmp::{aad_nonce, view};
use nonos_wifi_core::station::{Ccmp, LinkStation};

pub const STA: [u8; 6] = SPA;
pub const AP: [u8; 6] = AA;
pub const SRC: [u8; 6] = [0x02, 0, 0, 0, 0, 0x77];

// An AP-to-station frame: FromDS, QoS (TID 0) when `qos`, `body` from the
// LLC/SNAP header on, encrypted under `key` at `pn` with key index `kid`
// unless `key` is None.
pub fn from_ap(
    dst: [u8; 6],
    body: &[u8],
    key: Option<&[u8; 16]>,
    pn: u64,
    kid: u8,
    qos: bool,
) -> Vec<u8> {
    let mut f = vec![if qos { 0x88 } else { 0x08 }, 0x02, 0, 0];
    f.extend_from_slice(&dst);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&SRC);
    f.extend_from_slice(&[0x10, 0x00]);
    if qos {
        f.extend_from_slice(&[0x00, 0x00]);
    }
    let Some(key) = key else {
        f.extend_from_slice(body);
        return f;
    };
    f[1] |= 0x40;
    let hdr_len = f.len();
    let p = pn.to_be_bytes();
    f.extend_from_slice(&[p[7], p[6], 0, 0x20 | (kid << 6), p[5], p[4], p[3], p[2]]);
    let v = view(&f).unwrap();
    let mut aad = [0u8; 32];
    let (nonce, n) = aad_nonce(&f, &v, pn, &mut aad);
    let mut ct = vec![0u8; body.len() + 8];
    ccm_encrypt(key, &nonce, &aad[..n], body, &mut ct).unwrap();
    assert_eq!(hdr_len, v.hdr_len);
    f.extend_from_slice(&ct);
    f
}

pub fn station() -> LinkStation {
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::Software { tk: tk() });
    s
}
