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

//! Where a `MkMmioMap` request lands in physical pages. Pure, and proven on
//! the host (kernel_proofs::mmio_window).
//!
//! A BAR need not be a whole page, nor start on one: the AHCI ABAR of many
//! Intel PCH SATA controllers is 2 KiB and may sit at a base like
//! 0xF7E3_6800. Paging maps whole pages, so the broker maps every page the
//! request touches and hands the capsule a VA that already carries the
//! request's offset inside its first page. A page-aligned request (every
//! caller before sub-page BARs were allowed) is mapped exactly as it was.
//!
//! The pages around a sub-page request may hold registers that are not the
//! claimed device's: the rest of the page is either decode nobody claims
//! (reads all ones, writes dropped) or another function's small BAR. Linux
//! maps the same page for the same BAR (ioremap rounds to pages), and the
//! PCI spec asks for 4 KiB BARs for this reason, so the exposure is no wider
//! than on any other OS. What must never share a mapped page is an MSI-X
//! table or pending-bit array, of any device: those are the kernel's, and a
//! capsule writing an MSI-X entry could aim an interrupt message anywhere.
//! `protected` names them in physical addresses, and the mapping is cut
//! short at the page below the first one it would reach. A request whose
//! first page already holds one is refused.

pub const PAGE_SIZE: u64 = 4096;
const PAGE_MASK: u64 = PAGE_SIZE - 1;

/// The pages one request maps and where the caller's bytes sit in them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    /// The first physical page mapped, page aligned.
    pub page_start: u64,
    /// Bytes mapped from `page_start`, a whole number of pages.
    pub page_bytes: u64,
    /// Where the requested first byte sits in the first page.
    pub in_page: u64,
    /// Bytes from the requested first byte the caller may use: the request,
    /// or less when a protected region cut it short.
    pub usable: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowError {
    ZeroLength,
    Overflow,
    /// The request reaches past the end of the BAR.
    BadRange,
    /// Nothing of the request is left once the protected pages are kept out.
    Protected,
}

const fn floor(addr: u64) -> u64 {
    addr & !PAGE_MASK
}

const fn ceil(addr: u64) -> Option<u64> {
    match addr.checked_add(PAGE_MASK) {
        Some(a) => Some(a & !PAGE_MASK),
        None => None,
    }
}

/// The window for `length` bytes at `offset` into the BAR at `bar_base` of
/// `bar_size` bytes, kept off every page that holds a byte of a `protected`
/// half-open physical range.
pub fn window(
    bar_base: u64,
    bar_size: u64,
    offset: u64,
    length: u64,
    protected: &[(u64, u64)],
) -> Result<Window, WindowError> {
    if length == 0 {
        return Err(WindowError::ZeroLength);
    }
    let start = bar_base.checked_add(offset).ok_or(WindowError::Overflow)?;
    let mut end = start.checked_add(length).ok_or(WindowError::Overflow)?;
    let bar_end = bar_base.checked_add(bar_size).ok_or(WindowError::Overflow)?;
    if end > bar_end {
        return Err(WindowError::BadRange);
    }
    /*
     * Cutting the end back only ever drops pages, so a region that missed
     * the pages before a cut still misses them after it, and one pass over
     * the regions is enough.
     */
    for &(lo, hi) in protected {
        if lo >= hi {
            continue;
        }
        let pages_end = ceil(end).ok_or(WindowError::Overflow)?;
        if lo < pages_end && hi > floor(start) {
            end = end.min(floor(lo));
        }
    }
    if end <= start {
        return Err(WindowError::Protected);
    }
    let page_start = floor(start);
    let page_end = ceil(end).ok_or(WindowError::Overflow)?;
    Ok(Window {
        page_start,
        page_bytes: page_end - page_start,
        in_page: start - page_start,
        usable: end - start,
    })
}
