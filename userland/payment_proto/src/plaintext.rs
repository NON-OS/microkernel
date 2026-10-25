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

//! ORDER_PLAINTEXT (op 18, plan/29), from app.linux.install alone.
//! Request: orderId[32] | offset u64 LE. Reply: total u64 LE | offset u64 LE
//! | up to PLAINTEXT_PAGE bytes of the order's checked plaintext from offset.
//! Reads repeat: an install that dies halfway reads again. ENOENT before
//! ORDER_DELIVERED's OK.

use alloc::vec::Vec;

use crate::cursor::Cursor;

pub const PLAIN_ASK_LEN: usize = 40;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PlainAsk {
    pub order_id: [u8; 32],
    pub offset: u64,
}

impl PlainAsk {
    pub fn encode(&self) -> Vec<u8> {
        [&self.order_id[..], &self.offset.to_le_bytes()].concat()
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (order_id, offset) = (r.fixed()?, r.u64()?);
        r.end(PlainAsk { order_id, offset })
    }
}
