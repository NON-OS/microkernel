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

//! OP_CHANNEL_STATE. Request: chain_id u64 | channel_id[32]. Reply body:
//! deposited u128 | settled u128 | expiry u64 | closed u8 (0 or 1).

pub const CHANNEL_STATE_LEN: usize = 16 + 16 + 8 + 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChannelState {
    pub deposited: u128,
    pub settled: u128,
    pub expiry: u64,
    pub closed: bool,
}

pub fn channel_state_request(chain_id: u64, channel_id: &[u8; 32]) -> [u8; 40] {
    let mut out = [0u8; 40];
    out[..8].copy_from_slice(&chain_id.to_le_bytes());
    out[8..].copy_from_slice(channel_id);
    out
}

pub fn channel_state_reply(s: &ChannelState) -> [u8; CHANNEL_STATE_LEN] {
    let mut out = [0u8; CHANNEL_STATE_LEN];
    out[..16].copy_from_slice(&s.deposited.to_le_bytes());
    out[16..32].copy_from_slice(&s.settled.to_le_bytes());
    out[32..40].copy_from_slice(&s.expiry.to_le_bytes());
    out[40] = s.closed as u8;
    out
}

/// The reply body, or None unless it is exactly one, with closed 0 or 1.
pub fn parse_channel_state(b: &[u8]) -> Option<ChannelState> {
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
