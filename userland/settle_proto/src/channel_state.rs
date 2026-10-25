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

//! OP_CHANNEL_STATE. Request: chain_hash(chain_id, channel_id). Reply body:
//! deposited u128 | settled u128 | expiry u64 | closed u8 (0 or 1). Settle
//! reads settled from two RPC hosts and refuses when they disagree.

pub const CHANNEL_STATE_LEN: usize = 16 + 16 + 8 + 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChannelState {
    pub deposited: u128,
    pub settled: u128,
    pub expiry: u64,
    pub closed: bool,
}

impl ChannelState {
    pub fn encode(&self) -> [u8; CHANNEL_STATE_LEN] {
        let mut out = [0u8; CHANNEL_STATE_LEN];
        out[..16].copy_from_slice(&self.deposited.to_le_bytes());
        out[16..32].copy_from_slice(&self.settled.to_le_bytes());
        out[32..40].copy_from_slice(&self.expiry.to_le_bytes());
        out[40] = self.closed as u8;
        out
    }

    /// Exactly one body, with closed 0 or 1, or None.
    pub fn parse(b: &[u8]) -> Option<Self> {
        let b: &[u8; CHANNEL_STATE_LEN] = b.try_into().ok()?;
        let closed = match b[40] {
            0 => false,
            1 => true,
            _ => return None,
        };
        Some(ChannelState {
            deposited: u128::from_le_bytes(b[..16].try_into().ok()?),
            settled: u128::from_le_bytes(b[16..32].try_into().ok()?),
            expiry: u64::from_le_bytes(b[32..40].try_into().ok()?),
            closed,
        })
    }
}
