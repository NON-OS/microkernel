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

//! Each address space's two grant windows, MMIO and DMA (K10), found by
//! the asid its holder runs in, a thread's being its process's. Asids are
//! never reused, so a window outlives nothing it could be mistaken for. A
//! range goes back only after its pages are out of those tables.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::holder_space::holder_asid;
use super::va_window::VaWindow;
use super::windows::{USER_DMA_BASE, USER_DMA_END, USER_MMIO_BASE, USER_MMIO_END};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Region {
    Mmio,
    Dma,
}

struct Space {
    asid: u32,
    mmio: VaWindow,
    dma: VaWindow,
}

static SPACES: Mutex<Vec<Space>> = Mutex::new(Vec::new());

fn with_window<R>(pid: u32, region: Region, f: impl FnOnce(&mut VaWindow) -> R) -> Option<R> {
    let asid = holder_asid(pid)?;
    let mut spaces = SPACES.lock();
    let at = match spaces.iter().position(|s| s.asid == asid) {
        Some(at) => at,
        None => {
            let mmio = VaWindow::new(USER_MMIO_BASE, USER_MMIO_END);
            let dma = VaWindow::new(USER_DMA_BASE, USER_DMA_END);
            spaces.push(Space { asid, mmio, dma });
            spaces.len() - 1
        }
    };
    let space = &mut spaces[at];
    Some(f(if region == Region::Mmio { &mut space.mmio } else { &mut space.dma }))
}

/// `bytes` of `pid`'s window in `region`.
pub(crate) fn reserve(pid: u32, region: Region, bytes: u64) -> Option<u64> {
    with_window(pid, region, |w| w.reserve(bytes)).flatten()
}

/// A range back to `pid`'s window, its pages already unmapped.
pub(crate) fn release(pid: u32, region: Region, start: u64, bytes: u64) {
    with_window(pid, region, |w| w.release(start, bytes));
}
