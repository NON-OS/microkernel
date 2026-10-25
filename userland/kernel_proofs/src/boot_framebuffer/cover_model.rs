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

//! A second account of the cover rule over a small universe (L79): 64 pages,
//! so a span's pages are the bits of one u64, found page by page from the
//! bytes it covers, with no rounding at all. The seeded generator is our
//! own; nothing here comes from outside the crate.

use super::cover_find::PAGE;

/// Pages in the universe, and its size in bytes.
pub(super) const PAGES: u64 = 64;
pub(super) const END: u64 = PAGES * PAGE;
/// The step every generated base and size is a multiple of.
pub(super) const STEP: u64 = 0x400;

/// The pages any byte of `[base, base + len)` lies in, page by page.
pub(super) fn touched(base: u64, len: u64) -> u64 {
    let mut mask = 0;
    for p in 0..PAGES {
        let (lo, hi) = (p * PAGE, (p + 1) * PAGE);
        // Not empty, its first byte below the page's end, and its end past
        // the page's start.
        if len > 0 && base < hi && lo < base + len {
            mask |= 1 << p;
        }
    }
    mask
}

/// The bits of a page span `(first, end)` as `pages` gives it, which must
/// lie on page edges: its start always, its end unless it saturated at the
/// top of the address space. A span off its edges would be floored here
/// and pass unseen, so it fails instead.
pub(super) fn bits(span: (u64, u64)) -> u64 {
    let edges = span.0.is_multiple_of(PAGE) && (span.1.is_multiple_of(PAGE) || span.1 == u64::MAX);
    assert!(edges, "{span:x?} is not whole pages");
    (span.0 / PAGE..span.1.min(END) / PAGE).fold(0, |mask, p| mask | 1 << p)
}

/// A linear congruential generator, seeded per sweep.
pub(super) struct Lcg(pub(super) u64);

impl Lcg {
    pub(super) fn below(&mut self, n: u64) -> u64 {
        let (mul, add) = (6_364_136_223_846_793_005, 1_442_695_040_888_963_407);
        self.0 = self.0.wrapping_mul(mul).wrapping_add(add);
        (self.0 >> 33) % n
    }
}
