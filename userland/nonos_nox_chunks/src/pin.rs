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

//! The escrow install pin word (plan/27, v1.4.25), written by init's
//! linux_jobs in place of the BLAKE3 pin: `e` | 64 hex chunk_root | 16 hex
//! content_len (u64 big endian) | 64 hex orderId, 145 lowercase characters.
//! Nothing else parses: no upper case, no other length, no other lead.

pub const PIN_LEN: usize = 1 + 64 + 16 + 64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EscrowPin {
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    pub order_id: [u8; 32],
}

#[path = "pin_hex.rs"]
mod hex;

use hex::{put, take};

impl EscrowPin {
    pub fn word(&self) -> [u8; PIN_LEN] {
        let mut w = [0u8; PIN_LEN];
        w[0] = b'e';
        put(&mut w[1..65], &self.chunk_root);
        put(&mut w[65..81], &self.content_len.to_be_bytes());
        put(&mut w[81..], &self.order_id);
        w
    }

    pub fn parse(w: &[u8]) -> Option<Self> {
        let w: &[u8; PIN_LEN] = w.try_into().ok()?;
        (w[0] == b'e').then_some(())?;
        Some(EscrowPin {
            chunk_root: take(&w[1..65])?,
            content_len: u64::from_be_bytes(take(&w[65..81])?),
            order_id: take(&w[81..])?,
        })
    }
}
