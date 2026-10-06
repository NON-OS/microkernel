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

//! The memory window child BARs are placed in, chosen from the VMD's MEMBARs.

/// The first 8 KiB of MEMBAR2 hold the VMD's own MSI-X table and PBA.
pub const MEMBAR2_RESERVED: u64 = 0x2000;

pub(super) const MIB: u64 = 1 << 20;
pub(super) const FOUR_GIB: u64 = 1 << 32;

/// Where child BARs go, and how bridges are told so.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub base: u64,
    /// Inclusive last byte.
    pub limit: u64,
    /// The window sits above 4 GiB and is routed through the bridges'
    /// 64-bit prefetchable window; otherwise through the 32-bit memory one.
    pub prefetch64: bool,
}

/// The window child BARs are placed in. A bridge's ordinary memory window is
/// 32-bit, so a MEMBAR below 4 GiB is preferred: MEMBAR1 first, then what
/// MEMBAR2 leaves after the VMD's MSI-X pages. Only when neither fits below
/// 4 GiB is MEMBAR1 used through the prefetchable 64-bit window. Each MEMBAR
/// is (base, size); an absent one has size 0.
pub fn pick_window(membar1: (u64, u64), membar2: (u64, u64)) -> Option<Window> {
    let m2 = (membar2.0 + MEMBAR2_RESERVED, membar2.1.saturating_sub(MEMBAR2_RESERVED));
    for (base, size) in [membar1, m2] {
        if base == 0 || size < MIB {
            continue;
        }
        let limit = base + size - 1;
        if limit < FOUR_GIB {
            return Some(Window { base, limit, prefetch64: false });
        }
    }
    let (base, size) = membar1;
    if base != 0 && size >= MIB {
        return Some(Window { base, limit: base + size - 1, prefetch64: true });
    }
    None
}
