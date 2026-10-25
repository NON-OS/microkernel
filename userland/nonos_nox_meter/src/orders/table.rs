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

//! The orders file's plain form, sealed by the capsule under the bound root:
//! "NXOR" | version u16 = 2 | n u16 | MAX_ORDERS slots of ORDER_LEN, the
//! first n used and every byte of the rest zero. Fixed length, since the
//! store replaces a kept record only at its own length; anything else reads
//! as no orders file at all.

use alloc::vec::Vec;

use super::record::{Order, ORDER_LEN};

pub const MAX_ORDERS: usize = 32;
pub const ORDERS_LEN: usize = 8 + MAX_ORDERS * ORDER_LEN;
const MAGIC: [u8; 4] = *b"NXOR";
const VERSION: u16 = 2;

pub fn encode_orders(orders: &[Order]) -> Option<Vec<u8>> {
    if orders.len() > MAX_ORDERS {
        return None;
    }
    let mut out = Vec::with_capacity(ORDERS_LEN);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(orders.len() as u16).to_le_bytes());
    orders.iter().for_each(|o| o.encode(&mut out));
    out.resize(ORDERS_LEN, 0);
    Some(out)
}

pub fn decode_orders(b: &[u8]) -> Option<Vec<Order>> {
    if b.len() != ORDERS_LEN || b[..4] != MAGIC || b[4..6] != VERSION.to_le_bytes() {
        return None;
    }
    let n = u16::from_le_bytes([b[6], b[7]]) as usize;
    if n > MAX_ORDERS || b[8 + n * ORDER_LEN..].iter().any(|x| *x != 0) {
        return None;
    }
    b[8..8 + n * ORDER_LEN].chunks(ORDER_LEN).map(Order::decode).collect()
}
