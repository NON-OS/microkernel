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

//! A claimant's reserved regions in its domain (L60): where one may be mapped
//! one to one, and placement that steps over every one mapped. Pure, so the
//! host proofs hold it.

extern crate alloc;

use alloc::vec::Vec;

use super::avoid::place_avoiding;
use super::iova_space::{touches_interrupt_window, IOVA_BASE};

/// A reserved region `[base, end)` mapped for `device`'s sake.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Held {
    pub base: u64,
    pub end: u64,
    pub device: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Verdict {
    /// Mapped afresh, IOVA equal to its address.
    Map,
    /// Already mapped for another device of the capsule.
    Held,
    Refuse(&'static [u8]),
}

/// What becomes of region `[base, end)` in a capsule whose grants run from
/// IOVA_BASE to `next`, holding `held`; `barred` when it touches RAM or
/// another capsule's domain holds part of it (L63).
pub(super) fn verdict(base: u64, end: u64, next: u64, held: &[Held], barred: bool) -> Verdict {
    if barred {
        return Verdict::Refuse(b"refused: an RMRR over RAM or another capsule's region");
    }
    if end <= base || touches_interrupt_window(base, end - base) {
        return Verdict::Refuse(b"refused: an RMRR covers the interrupt window or nothing");
    }
    if held.iter().any(|h| h.base == base && h.end == end) {
        return Verdict::Held;
    }
    if held.iter().any(|h| h.base < end && base < h.end) {
        return Verdict::Refuse(b"refused: an RMRR overlaps another reserved region");
    }
    if next > IOVA_BASE && base < next && end > IOVA_BASE {
        return Verdict::Refuse(b"refused: an RMRR overlaps the capsule's grants");
    }
    Verdict::Map
}

/// The kernel's placement (`avoid::place_avoiding`) over the held regions
/// alone: it steps over each as over the interrupt window, so no grant's IOVA
/// lands on a region mapped one to one. The host proofs place by it; the
/// kernel passes these and the reserved ranges together (`reserved_spans`).
#[allow(dead_code)]
pub(super) fn place_clear(next: u64, length: u64, held: &[Held]) -> Option<(u64, u64)> {
    let spans: Vec<(u64, u64)> = held.iter().map(|h| (h.base, h.end)).collect();
    place_avoiding(next, length, &spans)
}
