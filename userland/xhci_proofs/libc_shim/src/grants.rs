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

//! The DMA grants live on the host, and where each sits on the bus.
//!
//! A grant's bus address is not its host address: each grant has its own
//! window above 4 GiB, so a driver that hands the controller a virtual
//! address, or keeps only the low half of a bus address, fails a test
//! instead of happening to work. A test plays the controller through
//! [`dma_host`], which turns a bus address back into the host memory behind
//! it, the way the controller's DMA would reach it.

use std::sync::Mutex;

pub(crate) const PAGE: u64 = 4096;
const BUS_BASE: u64 = 0x0000_00F0_0000_0000;
/// Room between grants on the bus. The driver asks for 16 pages at most.
pub(crate) const BUS_STRIDE: u64 = 1 << 20;

pub(crate) struct Grant {
    pub(crate) id: u64,
    pub(crate) host: u64,
    pub(crate) bus: u64,
    pub(crate) len: u64,
}

static GRANTS: Mutex<Vec<Grant>> = Mutex::new(Vec::new());

pub(crate) fn grants() -> std::sync::MutexGuard<'static, Vec<Grant>> {
    GRANTS.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The bus address grant `id` is placed at.
pub(crate) fn bus_of(id: u64) -> u64 {
    BUS_BASE + id * BUS_STRIDE
}

/// The host address behind bus address `bus`, or `None` when no live grant
/// covers it: what the controller reaches when it follows a pointer.
pub fn dma_host(bus: u64) -> Option<u64> {
    grants().iter().find(|g| bus >= g.bus && bus - g.bus < g.len).map(|g| g.host + (bus - g.bus))
}

/// The 8 bytes at bus address `bus`, read with the table held, or `None`
/// unless one live grant covers all 8 and they are 8-byte aligned: how the
/// device itself reads what it was pointed at (`device_dma`).
pub(crate) fn read64(bus: u64) -> Option<u64> {
    let all = grants();
    let g = all.iter().find(|g| bus >= g.bus && bus - g.bus < g.len)?;
    let at = bus - g.bus;
    if !bus.is_multiple_of(8) || at + 8 > g.len {
        return None;
    }
    // SAFETY: `g` is in the table and the table is held. A grant leaves the
    // table, under this lock, before its memory is freed (`mk_dma_unmap`),
    // so all of `g` is mapped for the read; the 8 bytes lie inside it and
    // are aligned, as the page-aligned host block and an aligned `bus` give.
    Some(unsafe { ((g.host + at) as *const u64).read_volatile() })
}
