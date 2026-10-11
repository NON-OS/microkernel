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

//! What a buyer's settle POSTs: chain_id u64 LE |
//! listing_key[32] | release_key[32] | buyer_pk[32] | lp(u16) the proof of
//! work, here exactly the plan's 36 bytes (nonce | effort | solution).
//! Anything else is refused whole.

use nonos_nox_chunks::POW_FIELD_LEN;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Asked {
    pub chain_id: u64,
    pub listing_key: [u8; 32],
    pub release_key: [u8; 32],
    pub buyer_pk: [u8; 32],
    pub pow: [u8; POW_FIELD_LEN],
}

pub const BODY_LEN: usize = 8 + 96 + 2 + POW_FIELD_LEN;

impl Asked {
    pub fn parse(b: &[u8]) -> Option<Self> {
        let b: &[u8; BODY_LEN] = b.try_into().ok()?;
        let key = |at: usize| b[at..at + 32].try_into().ok();
        let n = u16::from_le_bytes([b[104], b[105]]) as usize;
        (n == POW_FIELD_LEN).then_some(())?;
        Some(Asked {
            chain_id: u64::from_le_bytes(b[..8].try_into().ok()?),
            listing_key: key(8)?,
            release_key: key(40)?,
            buyer_pk: key(72)?,
            pow: b[106..].try_into().ok()?,
        })
    }
}
