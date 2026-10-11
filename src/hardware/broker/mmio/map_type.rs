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

//! The third step of a map, once the window is known: the memory type, from
//! the request, the BAR and the kernel's own mappings of those pages.

use super::memory_type::{choose, fold, KernelView, MemType, TypeRefusal};
use super::types::MmioMapError;
use super::window::Window;
use crate::hardware::broker::device::BAR_FLAG_PREFETCH;
use crate::hardware::broker::Bar;

pub(super) fn memory_type(flags: u32, bar: &Bar, w: &Window) -> Result<MemType, MmioMapError> {
    let prefetchable = bar.flags & BAR_FLAG_PREFETCH != 0;
    choose(flags, prefetchable, kernel_view(w.page_start, w.page_bytes)).map_err(|r| match r {
        TypeRefusal::UnknownFlags => MmioMapError::UnsupportedFlags,
        TypeRefusal::NotPrefetchable => MmioMapError::NotPrefetchable,
        TypeRefusal::Mixed => MmioMapError::MixedMemoryType,
    })
}

fn kernel_view(start: u64, bytes: u64) -> KernelView {
    let end = start.saturating_add(bytes);
    fold(crate::memory::mmio::list_regions().into_iter().filter_map(|r| {
        let lo = r.pa.as_u64();
        let hi = lo.saturating_add(r.size as u64);
        let touches = lo < end && hi > start;
        let wc = r.flags.write_combining;
        touches.then_some(if wc { MemType::WriteCombining } else { MemType::Uncached })
    }))
}
