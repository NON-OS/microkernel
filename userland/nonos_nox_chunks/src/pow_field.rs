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

//! FETCH 1's 36-byte solution field: nonce[16] | effort u32 BE |
//! the Equi-X solution[16], inside the request's existing lp(u16) field.

use nonos_equix::SOLUTION_BYTES;

pub const POW_FIELD_LEN: usize = 16 + 4 + SOLUTION_BYTES;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PowField {
    pub nonce: [u8; 16],
    pub effort: u32,
    pub solution: [u8; SOLUTION_BYTES],
}

impl PowField {
    pub fn encode(&self) -> [u8; POW_FIELD_LEN] {
        let mut out = [0u8; POW_FIELD_LEN];
        out[..16].copy_from_slice(&self.nonce);
        out[16..20].copy_from_slice(&self.effort.to_be_bytes());
        out[20..].copy_from_slice(&self.solution);
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let b: &[u8; POW_FIELD_LEN] = b.try_into().ok()?;
        let effort = u32::from_be_bytes(b[16..20].try_into().ok()?);
        Some(PowField {
            nonce: b[..16].try_into().ok()?,
            effort,
            solution: b[20..].try_into().ok()?,
        })
    }
}
