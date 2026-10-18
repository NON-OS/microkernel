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

//! A DMAR reserved memory region (RMRR, VT-d 3.x 8.4): memory firmware said
//! named devices DMA to, such as integrated graphics' stolen memory or a USB
//! controller's legacy buffer. Bring-up maps a device these ranges and
//! nothing else. Pure and allocation free, so the host proofs
//! include it unchanged; a structure that is not exactly sound is refused.

use super::dmar_scope::{read_scopes, scope_covers, Cover, UnitScope};

pub const RMRR_TYPE: u16 = 1;
const HEADER_LEN: usize = 24;
const PAGE: u64 = 4096;
/// The largest region believed: graphics stolen memory is at most 1 GiB.
pub const RMRR_MAX: u64 = 1 << 30;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reserved {
    pub base: u64,
    /// One past the last byte; page aligned.
    pub end: u64,
    /// The devices it is for; `include_all` is never set.
    pub scope: UnitScope,
}

/// One RMRR structure (`bytes` exactly its `length` bytes). `None` for a
/// short structure, another type, a base or limit not on a page, a limit
/// below the base, or a region over RMRR_MAX.
pub fn parse_rmrr(bytes: &[u8]) -> Option<Reserved> {
    let head = bytes.get(..HEADER_LEN)?;
    let word = |at: usize| u64::from_le_bytes(head[at..at + 8].try_into().unwrap_or([0; 8]));
    if u16::from_le_bytes([head[0], head[1]]) != RMRR_TYPE {
        return None;
    }
    let (base, limit) = (word(8), word(16));
    let end = limit.checked_add(1)?;
    if base % PAGE != 0 || end % PAGE != 0 || end <= base || end - base > RMRR_MAX {
        return None;
    }
    let mut scope = UnitScope::EMPTY;
    scope.segment = u16::from_le_bytes([head[6], head[7]]);
    read_scopes(bytes, HEADER_LEN, &mut scope);
    Some(Reserved { base, end, scope })
}

/// Whether `r` is for the device at `segment`/`bdf`. A scope that cannot be
/// resolved is not a yes: the device then gets no reserved range.
pub fn names<F>(r: &Reserved, segment: u16, bdf: (u8, u8, u8), bridge_buses: &F) -> bool
where
    F: Fn(u8, u8, u8) -> Option<(u8, u8)>,
{
    scope_covers(&r.scope, segment, bdf, bridge_buses) == Cover::Yes
}
