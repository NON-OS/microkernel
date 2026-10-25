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

//! RMRR (VT-d 3.x, 8.4): memory firmware keeps doing DMA to behind the
//! kernel's back, such as USB legacy keyboard emulation and graphics' stolen
//! memory, which the devices it names must still reach once a unit
//! translates. Pure, so the host proofs hold it; only a sound one is kept.

use super::dmar_scope::Scope;
use super::dmar_scope_walk::{names_off_pci, walk_scopes};

pub const RMRR_TYPE: u16 = 1;
const RMRR_HEADER_LEN: usize = 24;
pub const MAX_RMRR_SCOPES: usize = 8;
const PAGE_MASK: u64 = 0xFFF;
/// The largest region believed: graphics stolen memory is at most 1 GiB.
pub const RMRR_MAX: u64 = 1 << 30;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rmrr {
    pub segment: u16,
    pub base: u64,
    /// The last byte of the region, as the table states it.
    pub limit: u64,
    pub scopes: [Scope; MAX_RMRR_SCOPES],
    pub count: u8,
    pub truncated: bool,
    /// It also names a device that is not on PCI, an ACPI namespace device
    /// say, which no capsule's domain ever holds (L65).
    pub foreign: bool,
}

/// Parse one RMRR (`bytes` is exactly its `length` bytes, type 1). `None` for
/// a structure too short, or a region empty, inverted, not whole pages, past
/// the top of memory or over RMRR_MAX: firmware gets those wrong, and mapping
/// a guess is worse than leaving the region out and saying so.
pub fn parse_rmrr(bytes: &[u8]) -> Option<Rmrr> {
    if bytes.len() < RMRR_HEADER_LEN || u16::from_le_bytes([bytes[0], bytes[1]]) != RMRR_TYPE {
        return None;
    }
    let word = |at: usize| u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap_or([0; 8]));
    let (base, limit) = (word(8), word(16));
    let end = limit.checked_add(1)?;
    let sound = base & PAGE_MASK == 0 && limit & PAGE_MASK == PAGE_MASK && limit >= base;
    if !sound || end - base > RMRR_MAX {
        return None;
    }
    let mut scopes = [Scope::EMPTY; MAX_RMRR_SCOPES];
    let (count, truncated) = walk_scopes(bytes, RMRR_HEADER_LEN, &mut scopes);
    let segment = u16::from_le_bytes([bytes[6], bytes[7]]);
    let foreign = names_off_pci(bytes, RMRR_HEADER_LEN);
    Some(Rmrr { segment, base, limit, scopes, count, truncated, foreign })
}

/// The pages an RMRR names whatever else is wrong with it, rounded outward,
/// `[first, last]` byte (L67): one firmware got wrong still says a device
/// uses that memory. `None` when not even its header can be read.
pub fn rmrr_span(bytes: &[u8]) -> Option<(u64, u64)> {
    let word = |at: usize| Some(u64::from_le_bytes(bytes.get(at..at + 8)?.try_into().ok()?));
    let (a, b) = (word(8)?, word(16)?);
    Some((a.min(b) & !PAGE_MASK, a.max(b) | PAGE_MASK))
}
