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

//! The memory type a MkMmioMap mapping gets (L17), pure; kernel_proofs::
//! mmio_cache. Strong UC unless MAP_WC is asked. Write-combining merges
//! stores and reads ahead speculatively, so only a prefetchable BAR, whose
//! reads have no side effect, gets it, and only for pages wholly inside it.
//! With no WC entry in every CPU's PAT the mapping stays uncached.

/// Ask for write-combining: a framebuffer, VRAM.
pub(crate) const MAP_WC: u32 = 1 << 0;
pub(crate) const FLAGS_KNOWN: u32 = MAP_WC;
/// The prefetchable bit of a BAR's broker record (device/flags.rs).
pub(crate) const PREFETCH: u8 = 1 << 0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Cache {
    Uncached,
    WriteCombined,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Refused {
    /// A bit outside FLAGS_KNOWN.
    Unknown,
    /// MAP_WC on a BAR whose reads may have side effects.
    NotPrefetchable,
    /// MAP_WC for pages that reach past the BAR.
    PastBar,
}

pub(crate) const fn known(flags: u32) -> bool {
    flags & !FLAGS_KNOWN == 0
}

/// The type for the pages `(start, bytes)` of the BAR `(base, size, flags)`;
/// `wc_ready`: every CPU's PAT has its WC entry.
pub(crate) fn cache_for(
    flags: u32,
    bar: (u64, u64, u8),
    pages: (u64, u64),
    wc_ready: bool,
) -> Result<Cache, Refused> {
    if !known(flags) {
        return Err(Refused::Unknown);
    }
    if flags & MAP_WC == 0 {
        return Ok(Cache::Uncached);
    }
    let ((base, size, bar_flags), (start, bytes)) = (bar, pages);
    if bar_flags & PREFETCH == 0 {
        return Err(Refused::NotPrefetchable);
    }
    let inside = match (start.checked_add(bytes), base.checked_add(size)) {
        (Some(end), Some(top)) => base <= start && end <= top,
        _ => false,
    };
    if !inside {
        return Err(Refused::PastBar);
    }
    Ok(if wc_ready { Cache::WriteCombined } else { Cache::Uncached })
}
