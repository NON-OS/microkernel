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

//! Speaking to the station the way its callers do: the Wi-Fi control family
//! the panels send through `nonos_wifi_client`, and net_core's link protocol.

use crate::soft::rig::Station;
use nonos_wifi_core::netif::wire::{HDR_LEN, MAGIC_NNET, VERSION};
use nonos_wifi_core::netif::MAX_RESPONSE;
use nonos_wifi_core::softmac::engine::WIFI_MAGIC;

/// The reply body of control `op` (after the 10-octet header).
pub fn control(st: &mut Station, op: u16, body: &[u8]) -> Vec<u8> {
    let mut req = WIFI_MAGIC.to_le_bytes().to_vec();
    req.extend_from_slice(&op.to_le_bytes());
    req.extend_from_slice(&77u32.to_le_bytes());
    req.extend_from_slice(body);
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = st.handle(&req, &mut out);
    assert_eq!(&out[..10], &req[..10], "the reply repeats the header");
    out[10..n].to_vec()
}

pub fn connect(st: &mut Station, ssid: &[u8], pass: &[u8], flags: u8) -> Vec<u8> {
    let mut b = vec![ssid.len() as u8];
    b.extend_from_slice(ssid);
    b.push(pass.len() as u8);
    b.extend_from_slice(pass);
    b.push(flags);
    control(st, 1, &b)
}

pub fn code(reply: &[u8]) -> i32 {
    i32::from_le_bytes(reply[..4].try_into().unwrap())
}

/// The names in a scan reply.
pub fn scan_names(st: &mut Station) -> Vec<Vec<u8>> {
    let r = control(st, 3, &[]);
    let (mut names, mut at) = (Vec::new(), 13);
    for _ in 0..r[12] {
        let len = r[at + 2] as usize;
        names.push(r[at + 3..at + 3 + len].to_vec());
        at += 3 + len;
    }
    names
}

/// net_core's request `op`; the reply's status and payload after it.
pub fn netif(st: &mut Station, op: u16, payload: &[u8]) -> (i32, Vec<u8>) {
    let mut req = vec![0u8; HDR_LEN];
    req[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    req[4..6].copy_from_slice(&VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&op.to_le_bytes());
    req[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    req.extend_from_slice(payload);
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = st.handle(&req, &mut out);
    let status = i32::from_le_bytes(out[HDR_LEN..HDR_LEN + 4].try_into().unwrap());
    (status, out[HDR_LEN + 4..n].to_vec())
}
