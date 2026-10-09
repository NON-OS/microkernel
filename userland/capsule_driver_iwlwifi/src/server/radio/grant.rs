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

//! One broker DMA grant as a gen3 `Region`. The broker gives a network-class
//! device at most 64 pages per grant (`dma_page_limit_for_class`), so the
//! firmware and rings are spread over several grants as `plan` lays them out.
//! Every host access is checked against the grant's length with `span` before
//! a byte moves; one that does not fit is refused, never performed. A fence on
//! each side keeps the compiler from reordering a copy across the register
//! write that hands the memory to the device, or the read that follows the
//! device's index.

use core::sync::atomic::{fence, Ordering};

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut};

use crate::firmware::gen3::plan::{GRANT_MAX, PAGE};
use crate::firmware::gen3::region::{span, Region};

pub struct Grant {
    va: usize,
    dev: u64,
    len: usize,
    id: u64,
}

impl Grant {
    /// Map `len` bytes, rounded up to whole pages, for the device; `None`
    /// when that is over the broker's per-grant limit or the broker refuses.
    pub fn map(device_id: u64, claim_epoch: u64, len: usize) -> Option<Grant> {
        let bytes = len.checked_add(PAGE - 1)? / PAGE * PAGE;
        if bytes == 0 || bytes > GRANT_MAX {
            return None;
        }
        let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
        if mk_dma_map(device_id, claim_epoch, bytes as u64, 0, &mut out) < 0 {
            return None;
        }
        let mapped = usize::try_from(out.length).unwrap_or(0).min(bytes);
        let grant = Grant { va: out.user_va as usize, dev: out.device_addr, len: mapped, id: out.grant_id };
        if grant.va == 0 || grant.len < len {
            grant.unmap();
            return None;
        }
        Some(grant)
    }

    /// Give the grant back (only before the device has been told of it).
    pub fn unmap(self) {
        let _ = mk_dma_unmap(self.id);
    }
}

impl Region for Grant {
    fn len(&self) -> usize {
        self.len
    }

    fn dev(&self) -> u64 {
        self.dev
    }

    fn write(&self, off: usize, src: &[u8]) -> bool {
        let Some(r) = span(self.len, off, src.len()) else { return false };
        fence(Ordering::SeqCst);
        // SAFETY: the broker mapped `len` bytes at `va` for this process, and
        // `span` put `off..off + src.len()` inside them.
        unsafe { core::ptr::copy_nonoverlapping(src.as_ptr(), (self.va + r.start) as *mut u8, src.len()) };
        fence(Ordering::SeqCst);
        true
    }

    fn read(&self, off: usize, dst: &mut [u8]) -> bool {
        let Some(r) = span(self.len, off, dst.len()) else { return false };
        fence(Ordering::SeqCst);
        // SAFETY: as for `write`; the device may be writing elsewhere in the
        // grant, but this range is one the caller's index says is complete.
        unsafe { core::ptr::copy_nonoverlapping((self.va + r.start) as *const u8, dst.as_mut_ptr(), dst.len()) };
        fence(Ordering::SeqCst);
        true
    }
}
