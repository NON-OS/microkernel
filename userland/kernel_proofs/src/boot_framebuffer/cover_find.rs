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

//! The find the kernel runs once at boot (bootfb/cover.rs), restated over
//! test records (L79): each record asked whether its own BARs hold the
//! framebuffer, that answer folded over every record, the display functions
//! counted, and each record then judged by the kernel's own rule. The glue
//! tests read the kernel's text to hold it to this same shape.

use super::cover_rule::{covers, holds, Span};

/// The kernel's BAR kind codes, which the glue tests read in device/bar.rs.
pub(super) const NONE: u8 = 0;
pub(super) const MMIO: u8 = 1;
pub(super) const PIO: u8 = 2;
pub(super) const PAGE: u64 = 4096;
pub(super) const KIB: u64 = 1024;
pub(super) const MIB: u64 = 1024 * KIB;

/// A record as find reads it: six BAR slots, how many of them count, and
/// whether it is a PCI display function.
pub(super) struct Rec {
    pub(super) bars: [Span; 6],
    pub(super) count: u8,
    pub(super) display: bool,
}

impl Rec {
    /// The first slots filled from `used`, the rest empty, all of `used`
    /// counted.
    pub(super) fn new(used: &[Span], display: bool) -> Self {
        let mut bars = [(0, 0, NONE); 6];
        bars[..used.len()].copy_from_slice(used);
        Self { bars, count: used.len() as u8, display }
    }
}

/// Whether this record's own memory holds the framebuffer `fb`.
pub(super) fn mine(fb: (u64, u64), r: &Rec) -> bool {
    holds(fb, &r.bars, usize::from(r.count), MMIO)
}

/// The indices of the records whose claims take the scanout.
pub(super) fn find(fb: (u64, u64), recs: &[Rec]) -> Vec<usize> {
    let any = recs.iter().any(|r| mine(fb, r));
    let single = recs.iter().filter(|r| r.display).count() == 1;
    let by = |&i: &usize| covers(mine(fb, &recs[i]), any, recs[i].display, single);
    (0..recs.len()).filter(by).collect()
}
