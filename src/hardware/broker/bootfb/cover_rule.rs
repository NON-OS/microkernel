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

//! Whether a PCI function's memory holds the boot framebuffer (L79), as pure
//! rules `kernel_proofs` mounts. Judged on whole pages, not bytes: a holder
//! maps every page its request touches, so a BAR sharing only the
//! framebuffer's last page still reaches its pixels.

const PAGE: u64 = 4096;

/// `[base, base + len)` widened to whole pages, as `(first, end)`. An end
/// past the top of the address space reads as reaching everything above.
pub(crate) const fn pages(base: u64, len: u64) -> (u64, u64) {
    let first = base & !(PAGE - 1);
    let end = match base.checked_add(len) {
        Some(e) => match e.checked_add(PAGE - 1) {
            Some(up) => up & !(PAGE - 1),
            None => u64::MAX,
        },
        None => u64::MAX,
    };
    (first, end)
}

/// Two page spans share a page.
pub(crate) const fn meets(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 < b.1 && b.0 < a.1
}

/// A BAR as the rule reads it: its base, size and kind.
pub(crate) type Span = (u64, u64, u8);

/// Some memory BAR among the first `count` of `bars` shares a page with the
/// framebuffer at `fb` (its first byte and length). `mmio` is the memory
/// kind's code; a BAR of no size, or of ports, holds nothing.
pub(crate) fn holds(fb: (u64, u64), bars: &[Span], count: usize, mmio: u8) -> bool {
    let fb = pages(fb.0, fb.1);
    let live = |s: &&Span| s.2 == mmio && s.1 > 0;
    bars.iter().take(count).filter(live).any(|&(base, size, _)| meets(pages(base, size), fb))
}

/// Whether a claim of a function stops the kernel's console for good: its
/// own BAR holds the framebuffer, or no BAR anywhere does, the function is
/// a display and it is the only one, so the framebuffer can only be its
/// memory. With two displays and no BAR holding it, nobody is named.
pub(crate) const fn covers(mine: bool, any: bool, display: bool, single_display: bool) -> bool {
    mine || (!any && display && single_display)
}
