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

//! The register mapping once the driver gives it back: the adapter keeps
//! what the driver left in its registers, and the mapping reads poison until
//! the release, which finds any write made there since and puts them back.

use std::cell::{Cell, RefCell};

use crate::watch::registers;

const POISON: u8 = 0xA5;

thread_local! {
    static KEPT: RefCell<Option<Vec<u8>>> = const { RefCell::new(None) };
    static TOUCHED: Cell<bool> = const { Cell::new(false) };
}

/// Whether every release on this thread found the mapping as it was poisoned.
pub fn untouched_since_unmap() -> bool {
    !TOUCHED.with(Cell::get)
}

/// The watched registers as the adapter holds them.
pub(crate) fn adapter_holds() -> Vec<u8> {
    let Some((va, _, block)) = registers() else { return Vec::new() };
    let kept =
        KEPT.with(|k| k.borrow().as_ref().map(|k| k.get(block.clone()).unwrap_or(&[]).to_vec()));
    kept.unwrap_or_else(|| bytes(va, block))
}

/// The register mapping given back: its bytes kept, once, and it poisoned.
pub(crate) fn seal() {
    let Some((va, len, _)) = registers() else { return };
    if KEPT.with(|k| k.borrow().is_none()) {
        KEPT.with(|k| *k.borrow_mut() = Some(bytes(va, 0..len)));
        // SAFETY: as for `bytes`.
        unsafe { std::ptr::write_bytes(va as *mut u8, POISON, len) };
    }
}

/// The release: a register mapping given back is checked for writes since
/// its unmap, then holds the adapter's registers again.
pub(crate) fn unseal() {
    let (Some(kept), Some((va, len, _))) = (forget(), registers()) else { return };
    TOUCHED.with(|t| t.set(t.get() || bytes(va, 0..len).iter().any(|&b| b != POISON)));
    // SAFETY: as for `bytes`; `kept` was copied out of this mapping.
    unsafe { std::ptr::copy_nonoverlapping(kept.as_ptr(), va as *mut u8, kept.len().min(len)) };
}

/// What was kept of a register mapping given back, now dropped.
pub(crate) fn forget() -> Option<Vec<u8>> {
    KEPT.with(|k| k.take())
}

fn bytes(va: u64, at: std::ops::Range<usize>) -> Vec<u8> {
    let (mut b, from) = (vec![0; at.len()], (va as usize + at.start) as *const u8);
    // SAFETY: `at` lies in the mapping the test made with `map_bar`, memory
    // it holds until the driver has given its grants back.
    unsafe { std::ptr::copy_nonoverlapping(from, b.as_mut_ptr(), b.len()) };
    b
}
