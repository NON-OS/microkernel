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

//! One order as the meter keeps it: chain_id u64 | orderId[32] |
//! depositAddress[20] | the 14 term words | refundTo[20] | refundSalt[32] |
//! flags u8: bit 0, ORDER_REFUND has answered for it (v1.4.26); bit 1,
//! ORDER_DELIVERED has sealed its plaintext (v1.4.42). Older records hold
//! 0 or 1 there and read the same.

use alloc::vec::Vec;

pub const ORDER_LEN: usize = 8 + 32 + 20 + 32 * 14 + 20 + 32 + 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Order {
    pub chain_id: u64,
    pub order_id: [u8; 32],
    pub deposit: [u8; 20],
    pub terms: [[u8; 32]; 14],
    pub refund_to: [u8; 20],
    pub refund_salt: [u8; 32],
    pub refund_taken: bool,
    pub plain: bool,
}

impl Order {
    pub fn encode(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.chain_id.to_le_bytes());
        out.extend_from_slice(&self.order_id);
        out.extend_from_slice(&self.deposit);
        self.terms.iter().for_each(|w| out.extend_from_slice(w));
        out.extend_from_slice(&self.refund_to);
        out.extend_from_slice(&self.refund_salt);
        out.push(self.refund_taken as u8 | (self.plain as u8) << 1);
    }

    /// One record, or None for a short one, a flag past bit 1, or the zero
    /// chain an empty slot has.
    pub fn decode(b: &[u8]) -> Option<Order> {
        let b: &[u8; ORDER_LEN] = b.try_into().ok()?;
        let at = |from: usize, n: usize| &b[from..from + n];
        let chain_id = u64::from_le_bytes(at(0, 8).try_into().ok()?);
        let mut terms = [[0u8; 32]; 14];
        for (i, w) in terms.iter_mut().enumerate() {
            *w = at(60 + 32 * i, 32).try_into().ok()?;
        }
        let flags = b[560];
        (flags <= 3).then_some(())?;
        let (refund_taken, plain) = (flags & 1 != 0, flags & 2 != 0);
        (chain_id != 0).then_some(())?;
        Some(Order {
            chain_id,
            order_id: at(8, 32).try_into().ok()?,
            deposit: at(40, 20).try_into().ok()?,
            terms,
            refund_to: at(508, 20).try_into().ok()?,
            refund_salt: at(528, 32).try_into().ok()?,
            refund_taken,
            plain,
        })
    }
}
