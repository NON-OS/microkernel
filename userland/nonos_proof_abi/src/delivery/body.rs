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

//! A delivery body: an attested-execution record of one served request.
//! It shows what a measured service received and answered on a sealed
//! family's device; it is not a proof that the answer was computed right.

use alloc::vec::Vec;

pub const DELIVERY_BYTES: usize = 208;
const REQ_DOMAIN: &[u8] = b"NONOS-DELIVERY-REQ-v1";
const RSP_DOMAIN: &[u8] = b"NONOS-DELIVERY-RSP-v1";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Delivery {
    pub record_id: [u8; 32],
    pub service_meas: [u8; 32],
    pub req: [u8; 32],
    pub rsp: [u8; 32],
    pub order_id: [u8; 32],
    pub from: u64,
    pub to: u64,
    pub channel_id: [u8; 32],
}

fn digest(domain: &[u8], bytes: &[u8]) -> [u8; 32] {
    let mut b = Vec::with_capacity(domain.len() + bytes.len());
    b.extend_from_slice(domain);
    b.extend_from_slice(bytes);
    *blake3::hash(&b).as_bytes()
}

pub fn req_digest(request: &[u8]) -> [u8; 32] {
    digest(REQ_DOMAIN, request)
}

pub fn rsp_digest(response: &[u8]) -> [u8; 32] {
    digest(RSP_DOMAIN, response)
}
