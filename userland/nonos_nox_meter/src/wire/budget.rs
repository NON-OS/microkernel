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

//! SET_BUDGET's body, little-endian:
//!
//!   cap u128 | expiry u64 | n u8 | n x (listing_id[32] | cap u128)

use alloc::vec::Vec;

use crate::budget::Budget;
use crate::error::MeterError;

const ROW: usize = 32 + 16;

pub fn parse_budget(body: &[u8]) -> Result<Budget, MeterError> {
    let head = body.get(..25).ok_or(MeterError::Corrupt)?;
    let cap = u128::from_le_bytes(head[..16].try_into().map_err(|_| MeterError::Corrupt)?);
    let expiry = u64::from_le_bytes(head[16..24].try_into().map_err(|_| MeterError::Corrupt)?);
    let n = head[24] as usize;
    let rows = &body[25..];
    if rows.len() != n * ROW {
        return Err(MeterError::Corrupt);
    }
    let mut listings = Vec::with_capacity(n);
    for r in rows.chunks_exact(ROW) {
        let id: [u8; 32] = r[..32].try_into().map_err(|_| MeterError::Corrupt)?;
        let c = u128::from_le_bytes(r[32..].try_into().map_err(|_| MeterError::Corrupt)?);
        listings.push((id, c));
    }
    Budget::fresh(cap, expiry, &listings)
}
