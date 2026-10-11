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

//! The commerce block's canonical bytes: what the index carries, and with a
//! domain in front, what a release's `commerce_hash` is the BLAKE3 of.

extern crate alloc;

use alloc::vec::Vec;

use super::writer::Writer;
use crate::types::Commerce;

pub const COMMERCE_DOMAIN: &[u8] = b"NONOS.marketplace.commerce.v1";

pub fn commerce_bytes(c: &Commerce) -> Vec<u8> {
    let mut out = Vec::new();
    let mut w = Writer::new(&mut out);
    w.fixed(&c.payee);
    w.fixed(&c.bond_id);
    w.fixed(&c.unit);
    w.u128(c.unit_price);
    w.u128(c.min_charge);
    w.u8(c.settle_mode as u8);
    w.u32(c.proof_reqs.len() as u32);
    for req in &c.proof_reqs {
        w.u8(req.kind);
        w.fixed(&req.statement);
    }
    w.fixed(&c.binding_sig);
    out
}

/// `COMMERCE_DOMAIN | commerce_bytes`. The caller hashes it with BLAKE3; this
/// crate holds no hash so it stays the same on the host and in a capsule.
pub fn commerce_hash_input(c: &Commerce) -> Vec<u8> {
    let mut out = Vec::from(COMMERCE_DOMAIN);
    out.extend_from_slice(&commerce_bytes(c));
    out
}
