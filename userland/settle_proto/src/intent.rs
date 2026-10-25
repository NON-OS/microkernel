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

//! OP_INTENT. Request: chain_id u64 | kind u8 | from[20] | value u128 |
//! lp(u32) calldata. settle checks that the calldata's selector belongs to the
//! kind and quotes it (Quote); it returns no calldata and never changes any.

use alloc::vec::Vec;

use crate::bytes::{fixed, len_u32};

/// The largest calldata INTENT takes.
pub const MAX_CALLDATA: usize = 32 * 1024;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Intent {
    pub chain_id: u64,
    pub kind: u8,
    pub from: [u8; 20],
    pub value: u128,
    pub calldata: Vec<u8>,
}

impl Intent {
    /// None past MAX_CALLDATA, which parse would refuse; under it the u32
    /// length always fits.
    pub fn encode(&self) -> Option<Vec<u8>> {
        if self.calldata.len() > MAX_CALLDATA {
            return None;
        }
        let len = len_u32(self.calldata.len())?;
        let mut out = Vec::with_capacity(self.calldata.len().saturating_add(49));
        out.extend_from_slice(&self.chain_id.to_le_bytes());
        out.push(self.kind);
        out.extend_from_slice(&self.from);
        out.extend_from_slice(&self.value.to_le_bytes());
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&self.calldata);
        Some(out)
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let len = usize::try_from(u32::from_le_bytes(fixed(b, 45)?)).ok()?;
        if len > MAX_CALLDATA || Some(b.len()) != len.checked_add(49) {
            return None;
        }
        Some(Intent {
            chain_id: u64::from_le_bytes(fixed(b, 0)?),
            kind: *b.get(8)?,
            from: fixed(b, 9)?,
            value: u128::from_le_bytes(fixed(b, 29)?),
            calldata: b.get(49..)?.to_vec(),
        })
    }
}
