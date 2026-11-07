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

//! A surface's frames as physical runs, and the map of those runs at one
//! IOVA run with its undo, pure over `Space`: kernel_proofs::surface_import
//! runs it against a model domain, the kernel against the capsule's.

extern crate alloc;

use alloc::vec::Vec;

pub(crate) const PAGE: u64 = 4096;

/// Adjacent frames folded into `(phys, len)` runs, in page order.
pub(crate) fn runs(frames: impl Iterator<Item = u64>) -> Vec<(u64, u64)> {
    let mut out: Vec<(u64, u64)> = Vec::new();
    for f in frames {
        match out.last_mut() {
            Some((p, l)) if p.checked_add(*l) == Some(f) => *l += PAGE,
            _ => out.push((f, PAGE)),
        }
    }
    out
}

/// What one map of a run left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Step {
    Mapped,
    /// Nothing of the run is mapped.
    Refused,
    /// Entries written, the unit's sight of them not confirmed.
    Uncommitted,
    /// The unit may hold the run and will not give it up.
    Stranded,
}

/// A device's domain, read only.
pub(crate) trait Space {
    fn map(&mut self, iova: u64, phys: u64, len: u64) -> Step;
    /// True once the range is gone from the device's view, its IOTLB
    /// entries invalidated and the invalidation waited on.
    fn unmap(&mut self, iova: u64, len: u64) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mapped {
    /// Every run is mapped, contiguous from the IOVA.
    All,
    /// Nothing is mapped: the IOVA may go back.
    Undone,
    /// Part may still be reachable: the frames are quarantined and the IOVA
    /// kept.
    Stranded,
}

/// Map `runs` one after another from `iova`. A refusal takes back what was
/// mapped before it, so the domain holds all of the surface or none.
pub(crate) fn map_runs(space: &mut impl Space, iova: u64, runs: &[(u64, u64)]) -> Mapped {
    let mut at = iova;
    for &(phys, len) in runs {
        match space.map(at, phys, len) {
            Step::Mapped => at = at.saturating_add(len),
            Step::Refused => return undo(space, iova, at - iova),
            Step::Uncommitted if space.unmap(at, len) => return undo(space, iova, at - iova),
            Step::Uncommitted | Step::Stranded => return Mapped::Stranded,
        }
    }
    Mapped::All
}

fn undo(space: &mut impl Space, iova: u64, len: u64) -> Mapped {
    if len == 0 || space.unmap(iova, len) {
        Mapped::Undone
    } else {
        Mapped::Stranded
    }
}
