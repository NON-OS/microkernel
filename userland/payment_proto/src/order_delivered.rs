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

//! ORDER_DELIVERED (op 10, the plan, the plan), from the wallet or the
//! installer; a job (op 15). Request: orderId[32] | K[32]. Result: 0 |
//! content_len u64 once every chunk checked (op 18 then serves the
//! plaintext), or 1 | chunk index u64 | dispute()'s calldata, the rest.

use alloc::vec::Vec;

use crate::cursor::Cursor;

pub const DELIVERED_ASK_LEN: usize = 64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DeliveredAsk {
    pub order_id: [u8; 32],
    pub key: [u8; 32],
}

impl DeliveredAsk {
    pub fn encode(&self) -> Vec<u8> {
        [self.order_id, self.key].concat()
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (order_id, key) = (r.fixed()?, r.fixed()?);
        r.end(DeliveredAsk { order_id, key })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DeliveredMade {
    Ok { content_len: u64 },
    BadChunk { index: u64, calldata: Vec<u8> },
}

impl DeliveredMade {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            DeliveredMade::Ok { content_len } => [&[0u8][..], &content_len.to_le_bytes()].concat(),
            DeliveredMade::BadChunk { index, calldata } => {
                [&[1u8][..], &index.to_le_bytes(), calldata].concat()
            }
        }
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (tag, n) = (r.u8()?, r.u64()?);
        match tag {
            0 => r.end(DeliveredMade::Ok { content_len: n }),
            1 if b.len() > 9 => {
                Some(DeliveredMade::BadChunk { index: n, calldata: b[9..].to_vec() })
            }
            _ => None,
        }
    }
}
