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

//! A delivery body (plan/14): an attested-execution record of one served
//! request. It shows what a measured service received and answered on a
//! sealed family's device; it is not a proof that the answer was computed
//! right. The buyer's nonce salts every digest of the buyer's own data, so
//! the body names no request, answer, order or channel to anyone without it.

pub const DELIVERY_BYTES: usize = 208;
const REQ_DOMAIN: &[u8] = b"NONOS-DELIVERY-REQ-v1";
const RSP_DOMAIN: &[u8] = b"NONOS-DELIVERY-RSP-v1";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Delivery {
    pub record_id: [u8; 32],
    pub service_meas: [u8; 32],
    pub req: [u8; 32],
    pub rsp: [u8; 32],
    /// BLAKE3(nonce | order_id).
    pub order_ref: [u8; 32],
    pub from: u64,
    pub to: u64,
    /// BLAKE3(nonce | channel_id).
    pub channel_ref: [u8; 32],
}

fn digest(parts: &[&[u8]]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    parts.iter().for_each(|p| {
        h.update(p);
    });
    *h.finalize().as_bytes()
}

/// BLAKE3("NONOS-DELIVERY-REQ-v1" | nonce | request).
pub fn req_digest(nonce: &[u8; 32], request: &[u8]) -> [u8; 32] {
    digest(&[REQ_DOMAIN, nonce, request])
}

/// BLAKE3("NONOS-DELIVERY-RSP-v1" | nonce | response).
pub fn rsp_digest(nonce: &[u8; 32], response: &[u8]) -> [u8; 32] {
    digest(&[RSP_DOMAIN, nonce, response])
}

/// order_ref and channel_ref: BLAKE3(nonce | id).
pub fn nonce_ref(nonce: &[u8; 32], id: &[u8; 32]) -> [u8; 32] {
    digest(&[nonce, id])
}
