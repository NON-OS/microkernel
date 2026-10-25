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
//! What OP_SIGN_TX carries, read and checked before any key is touched: the
//! whole frame its own data length names, and a chain the wallet offers.

use super::super::field32::field32;

/// pid, wallet, chain id, to, nonce, tip, fee cap, gas, value, data length.
const HDR: usize = 4 + 4 + 8 + 20 + 32 * 5 + 4;
/// A settlement carries about 99 KB of proof; nothing the wallet sends is larger.
pub const MAX_DATA: usize = 128 * 1024;

pub(super) struct Tx<'a> {
    pub payload_pid: u32,
    pub id: u32,
    pub chain_be: [u8; 8],
    pub to: [u8; 20],
    pub nonce: [u8; 32],
    pub max_priority: [u8; 32],
    pub max_fee: [u8; 32],
    pub gas: [u8; 32],
    pub value: [u8; 32],
    pub data: &'a [u8],
}

/// The request, or None for a frame of the wrong size or a chain the wallet
/// does not offer, each answered EINVAL.
pub(super) fn parse(p: &[u8]) -> Option<Tx<'_>> {
    if p.len() < HDR {
        return None;
    }
    let data_len = u32::from_le_bytes([p[HDR - 4], p[HDR - 3], p[HDR - 2], p[HDR - 1]]) as usize;
    if data_len > MAX_DATA || p.len() != HDR + data_len {
        return None;
    }
    let mut chain = [0u8; 8];
    chain.copy_from_slice(&p[8..16]);
    let chain_id = u64::from_le_bytes(chain);
    // The networks the wallet offers, from the one table every capsule reads.
    nonos_nox_config::Chain::from_id(chain_id)?;
    let mut to = [0u8; 20];
    to.copy_from_slice(&p[16..36]);
    Some(Tx {
        payload_pid: u32::from_le_bytes([p[0], p[1], p[2], p[3]]),
        id: u32::from_le_bytes([p[4], p[5], p[6], p[7]]),
        chain_be: chain_id.to_be_bytes(),
        to,
        nonce: field32(p, 36),
        max_priority: field32(p, 68),
        max_fee: field32(p, 100),
        gas: field32(p, 132),
        value: field32(p, 164),
        data: &p[HDR..],
    })
}
