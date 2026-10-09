// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Where the boot identity reaches a framebuffer, with no UEFI types so the
//! host proofs run this exact file.

use super::constants::{IDENTITY_LOW_BYTES, PAGE_SIZE};

/// The top of the canonical low half. An identity mapping of the
/// framebuffer is made anywhere below it; the kernel's low-half teardown
/// clears every low-half PML4 slot, so it goes with the rest of the boot
/// identity. A 64 bit BAR at 1 TiB or more (AMD and NVIDIA cards with
/// resizable BAR) is still reachable this way.
pub const IDENTITY_FB_LIMIT: u64 = 1 << 47;

/// The part of the framebuffer [base, base + len) the low identity window
/// misses, page aligned, or None when the window already covers it or the
/// framebuffer lies past what can be identity mapped and torn down.
pub fn framebuffer_tail(base: u64, len: u64) -> Option<(u64, u64)> {
    if base == 0 || len == 0 {
        return None;
    }
    let end = base.checked_add(len)?;
    let end = end.checked_add(PAGE_SIZE - 1)? & !(PAGE_SIZE - 1);
    if end <= IDENTITY_LOW_BYTES || end > IDENTITY_FB_LIMIT {
        return None;
    }
    let start = (base & !(PAGE_SIZE - 1)).max(IDENTITY_LOW_BYTES);
    Some((start, end - start))
}

/// Whether a framebuffer [base, base + len) is reachable through the boot
/// identity once the kernel PML4 is live: the low window, plus whatever
/// map_framebuffer_identity added past it.
pub fn framebuffer_identity_reachable(base: u64, len: u64) -> bool {
    match base.checked_add(len) {
        Some(end) => base != 0 && end <= IDENTITY_FB_LIMIT,
        None => false,
    }
}
