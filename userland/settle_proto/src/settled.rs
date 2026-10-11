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

//! OP_SETTLE (op 10, the plan v1.4.7): one drain round now. Request: chain_id
//! u64. Reply body: tx_hash[32] | n u8 | n x (channel_id[32] | cumulative
//! u128), the settle() it broadcast and the vouchers that transaction
//! carries, as the meter signed them. ENOENT when nothing is owed.

use alloc::vec::Vec;

/// The most channels one settle() carries: the meter's DRAIN_ROWS_MAX.
pub const SETTLED_MAX: usize = 12;
const ROW: usize = 32 + 16;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Settled {
    pub tx_hash: [u8; 32],
    pub carried: Vec<([u8; 32], u128)>,
}

impl Settled {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(33 + ROW * self.carried.len());
        out.extend_from_slice(&self.tx_hash);
        out.push(self.carried.len() as u8);
        for (id, cumulative) in &self.carried {
            out.extend_from_slice(id);
            out.extend_from_slice(&cumulative.to_le_bytes());
        }
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let tx_hash = b.get(..32)?.try_into().ok()?;
        let n = *b.get(32)? as usize;
        if n == 0 || n > SETTLED_MAX || b.len() != 33 + ROW * n {
            return None;
        }
        let row = |r: &[u8]| {
            Some((r[..32].try_into().ok()?, u128::from_le_bytes(r[32..].try_into().ok()?)))
        };
        let carried = b[33..].chunks_exact(ROW).map(row).collect::<Option<Vec<_>>>()?;
        Some(Settled { tx_hash, carried })
    }

    /// Whether this settle() carries `channel` at `cumulative` or above: the
    /// installer's test that its voucher went with it.
    pub fn carries(&self, channel: &[u8; 32], cumulative: u128) -> bool {
        self.carried.iter().any(|(id, c)| id == channel && *c >= cumulative)
    }
}
