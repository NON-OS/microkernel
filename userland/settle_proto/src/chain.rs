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

//! The request body most ops share: chain_id u64 | a 32-byte id (a channel
//! for CHANNEL_STATE, a transaction hash for OPENED, a listing for LISTING).
//! GAS names an address.

#[path = "chain_head.rs"]
mod head;
pub use head::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChainHash {
    pub chain_id: u64,
    pub id: [u8; 32],
}

pub fn chain_hash(chain_id: u64, id: &[u8; 32]) -> [u8; 40] {
    let mut out = [0u8; 40];
    out[..8].copy_from_slice(&chain_id.to_le_bytes());
    out[8..].copy_from_slice(id);
    out
}

pub fn parse_chain_hash(b: &[u8]) -> Option<ChainHash> {
    let b: &[u8; 40] = b.try_into().ok()?;
    Some(ChainHash {
        chain_id: u64::from_le_bytes(b[..8].try_into().ok()?),
        id: b[8..].try_into().ok()?,
    })
}
