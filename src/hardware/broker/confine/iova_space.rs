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

//! Where in a capsule's device address space a grant may sit. Pure, so the
//! placement rules are proven on the host (kernel_proofs::iova_space).
//!
//! The rule that is easy to miss: a device write to 0xFEEx_xxxx is not DMA.
//! The root complex takes it as a message signalled interrupt before any
//! translation happens, with or without a remapping unit. A grant whose IOVA
//! fell in that window would turn every buffer write the device made into an
//! interrupt aimed at whatever vector the low bytes spelled, and the buffer
//! would never be written. The bump pointer reaches the window only after
//! about four gigabytes of grants, which a GPU's framebuffers and churned
//! staging buffers can do, so placement steps over it.

/// Device addresses start above the first megabyte, so a driver that hands a
/// device a zero or small address faults instead of hitting a grant.
pub(super) const IOVA_BASE: u64 = 0x10_0000;
/// Every grant sits below 4 GiB, so a 32-bit descriptor can name any of them.
pub(super) const IOVA_LIMIT: u64 = 1 << 32;
/// The interrupt address range (Intel SDM vol. 3 "Message Address Register
/// Format", VT-d spec section 3.14): requests here are interrupts.
pub(super) const INTERRUPT_WINDOW_START: u64 = 0xFEE0_0000;
pub(super) const INTERRUPT_WINDOW_END: u64 = 0xFEF0_0000;

/// True if `[start, start + length)` touches the interrupt window.
pub(super) const fn touches_interrupt_window(start: u64, length: u64) -> bool {
    let end = match start.checked_add(length) {
        Some(e) => e,
        None => return true,
    };
    start < INTERRUPT_WINDOW_END && end > INTERRUPT_WINDOW_START
}

/// Place a run of `length` bytes at or above the bump pointer `next`. Returns
/// the run's start and the new bump pointer, or `None` when it does not fit
/// below the limit. A run that would overlap the interrupt window starts just
/// past it instead; the gap below stays unused, as any bump allocator's does.
/// The start is never below `next`.
pub(super) const fn place(next: u64, length: u64) -> Option<(u64, u64)> {
    if length == 0 {
        return None;
    }
    let mut start = if next < IOVA_BASE { IOVA_BASE } else { next };
    // Checked before the window test, so a run that wraps is refused rather
    // than read as touching the window and moved below where it was asked.
    if start.checked_add(length).is_none() {
        return None;
    }
    if touches_interrupt_window(start, length) {
        start = INTERRUPT_WINDOW_END;
    }
    let end = match start.checked_add(length) {
        Some(e) => e,
        None => return None,
    };
    if end > IOVA_LIMIT {
        return None;
    }
    Some((start, end))
}

/// The bump pointer after the run `[iova, iova + length)` is given back: it
/// moves down only when that run is the top one.
pub(super) const fn after_give_back(next: u64, iova: u64, length: u64) -> u64 {
    match iova.checked_add(length) {
        Some(end) if end == next => iova,
        _ => next,
    }
}
