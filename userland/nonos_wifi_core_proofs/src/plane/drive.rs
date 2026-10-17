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

//! net_core's four link operations against any soft-MAC chip: build the
//! request, read the status and body. Generic over the chip so the same
//! checks run over the plain air and over the chip that restarts.

use crate::soft::env::SimEnv;
use nonos_wifi_core::netif::wire::{HDR_LEN, MAGIC_NNET, OP_LINK_STATUS};
use nonos_wifi_core::netif::wire::{OP_RX_PACKET, OP_TX_PACKET, VERSION};
use nonos_wifi_core::netif::MAX_RESPONSE;
use nonos_wifi_core::softmac::{Radio, SoftStation};

fn call<R: Radio>(st: &mut SoftStation<R, SimEnv>, op: u16, payload: &[u8]) -> (i32, Vec<u8>) {
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

/// The link state net_core reads.
pub fn up<R: Radio>(st: &mut SoftStation<R, SimEnv>) -> bool {
    call(st, OP_LINK_STATUS, &[]).1.first() == Some(&1)
}

/// Offer one frame marked with `tag` to transmit; the status net_core sees.
pub fn tx<R: Radio>(st: &mut SoftStation<R, SimEnv>, tag: u8) -> i32 {
    let eth = [&[2, 0, 0, 0, 0, 9][..], &[2, 0, 0, 0, 0, 2], &[8, 0], &[tag; 24]].concat();
    call(st, OP_TX_PACKET, &eth).0
}

/// Poll for one received frame; whether a frame body came back.
pub fn rx<R: Radio>(st: &mut SoftStation<R, SimEnv>) -> bool {
    call(st, OP_RX_PACKET, &[]).1.len() > 4
}
