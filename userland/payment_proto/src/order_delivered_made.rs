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

//! ORDER_DELIVERED's result (op 10, the plan): 0 | content_len u64 once
//! every chunk checked, or 1 | chunk index u64 | dispute()'s calldata.

use alloc::vec::Vec;

use crate::cursor::Cursor;

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
