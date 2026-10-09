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

//! A broker in host memory. Config reads come from a snapshot, each memory
//! BAR is backed by zeroed page-aligned host memory of its size, every map
//! request is recorded, and a map can be made to fail or come back short
//! the way the kernel's MSI-X clamp makes it.

use std::alloc::{alloc_zeroed, dealloc, Layout};

use super::space::Space;
use crate::broker::{Broker, MmioGrant};
use crate::pci::{Bars, CONFIG_SPACE_LEN};

const PAGE: u64 = 4096;

struct HostBar {
    ptr: *mut u8,
    layout: Layout,
}

pub struct FakeBroker {
    cfg: [u8; CONFIG_SPACE_LEN],
    bars: Vec<Option<HostBar>>,
    /// Every map request, as (bar, offset, length).
    pub requests: Vec<(u8, u64, u64)>,
    pub live: Vec<u64>,
    pub unmapped: Vec<u64>,
    /// Refuse the map request with this index.
    pub refuse_map: Option<usize>,
    /// Map one page less than asked on the request with this index.
    pub short_map: Option<usize>,
    /// Refuse the config read at this offset.
    pub refuse_read: Option<u32>,
    pub reads: usize,
    next_grant: u64,
}

impl FakeBroker {
    pub fn new(space: &Space, bars: &Bars) -> Self {
        let bars = bars
            .iter()
            .map(|b| {
                if !b.is_mmio() {
                    return None;
                }
                let size = b.size.div_ceil(PAGE) * PAGE;
                let layout = Layout::from_size_align(size as usize, PAGE as usize).ok()?;
                // SAFETY: the layout has a non-zero size.
                let ptr = unsafe { alloc_zeroed(layout) };
                (!ptr.is_null()).then_some(HostBar { ptr, layout })
            })
            .collect();
        Self {
            cfg: space.bytes(),
            bars,
            requests: Vec::new(),
            live: Vec::new(),
            unmapped: Vec::new(),
            refuse_map: None,
            short_map: None,
            refuse_read: None,
            reads: 0,
            next_grant: 100,
        }
    }

    /// Host address of BAR `bar`'s backing, for checking where accesses land.
    pub fn bar_base(&self, bar: u8) -> usize {
        self.bars[bar as usize].as_ref().map_or(0, |b| b.ptr as usize)
    }

    pub fn bar_len(&self, bar: u8) -> usize {
        self.bars[bar as usize].as_ref().map_or(0, |b| b.layout.size())
    }
}

impl Drop for FakeBroker {
    fn drop(&mut self) {
        for bar in self.bars.iter().flatten() {
            // SAFETY: allocated in `new` with this layout.
            unsafe { dealloc(bar.ptr, bar.layout) };
        }
    }
}

// SAFETY: a grant points into a host allocation of at least the BAR's size
// that lives as long as the broker; tests keep the broker alive while they
// use a window.
unsafe impl Broker for FakeBroker {
    fn config_read32(&mut self, offset: u32) -> Option<u32> {
        self.reads += 1;
        if self.refuse_read == Some(offset) || !offset.is_multiple_of(4) {
            return None;
        }
        let b = self.cfg.get(offset as usize..offset as usize + 4)?;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn mmio_map(&mut self, bar: u8, offset: u64, length: u64) -> Result<MmioGrant, i64> {
        let index = self.requests.len();
        self.requests.push((bar, offset, length));
        if self.refuse_map == Some(index) {
            return Err(-1);
        }
        if length == 0 || !offset.is_multiple_of(PAGE) || !length.is_multiple_of(PAGE) {
            return Err(-22);
        }
        let Some(Some(host)) = self.bars.get(bar as usize) else {
            return Err(-19);
        };
        if offset + length > host.layout.size() as u64 {
            return Err(-22);
        }
        let mapped = if self.short_map == Some(index) { length - PAGE } else { length };
        self.next_grant += 1;
        self.live.push(self.next_grant);
        Ok(MmioGrant {
            user_va: host.ptr as u64 + offset,
            length: mapped,
            grant_id: self.next_grant,
        })
    }

    fn mmio_unmap(&mut self, grant_id: u64) -> bool {
        match self.live.iter().position(|&g| g == grant_id) {
            Some(i) => {
                self.live.remove(i);
                self.unmapped.push(grant_id);
                true
            }
            None => false,
        }
    }
}
