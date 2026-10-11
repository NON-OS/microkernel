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

//! The other ORDER bodies:
//!
//!   ORDER_NEW's result     orderId[32] | depositAddress[20] | 14 term words
//!   ORDER_STATUS orderId   -> state u8 | funded u128 | pending u128 | plain u8
//!                             (a job; plain 1 once ORDER_DELIVERED sealed it)
//!   ORDER_DELIVERED        orderId[32] | K[32] (a job)
//!   ORDER_REFUND orderId   -> refundTo[20] | refundSalt[32]

use alloc::vec::Vec;

use crate::cursor::Cursor;

pub const ORDER_STATUS_LEN: usize = 1 + 16 + 16 + 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OrderStatus {
    pub state: u8,
    pub funded: u128,
    /// NOX at the deposit address plus what the pool credits it.
    pub pending: u128,
    /// ORDER_DELIVERED has sealed the plaintext, so op 18 serves it.
    pub plain: bool,
}

impl OrderStatus {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::from([self.state]);
        out.extend_from_slice(&self.funded.to_le_bytes());
        out.extend_from_slice(&self.pending.to_le_bytes());
        out.push(self.plain as u8);
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (state, funded, pending) = (r.u8()?, r.u128()?, r.u128()?);
        let plain = crate::cursor_put::flag(r.u8()?)?;
        r.end(OrderStatus { state, funded, pending, plain })
    }
}

/// A refund's committed destination and salt, for the wallet alone.
pub fn refund_reply(refund_to: &[u8; 20], salt: &[u8; 32]) -> Vec<u8> {
    [&refund_to[..], salt].concat()
}

pub fn parse_refund_reply(b: &[u8]) -> Option<([u8; 20], [u8; 32])> {
    let mut r = Cursor::new(b);
    let (to, salt) = (r.fixed()?, r.fixed()?);
    r.end((to, salt))
}
