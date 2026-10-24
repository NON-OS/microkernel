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

//! Which of a domain's devices name each reserved range it holds (the
//! C-41 ruling): a range stays mapped while some attached device names it,
//! and is unmapped at the last one's detach, not when the domain drops.
//! Pure, so the proofs run it against a model of the domain.

extern crate alloc;

use alloc::vec::Vec;

use crate::memory::iommu::reserved::ReservedRange;
use crate::memory::iommu::DeviceAddress;

/// A range the domain holds, and the attached devices naming it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Held {
    pub(super) range: ReservedRange,
    pub(super) users: Vec<DeviceAddress>,
}

/// Of `ranges`, those the domain does not hold yet, each once, which must
/// be admitted and mapped before `name` records them. A range a firmware
/// table lists twice is one range, not two that overlap.
pub(super) fn unheld(held: &[Held], ranges: &[ReservedRange]) -> Vec<ReservedRange> {
    let mut out: Vec<ReservedRange> = Vec::new();
    for r in ranges {
        if !held.iter().any(|h| h.range == *r) && !out.contains(r) {
            out.push(*r);
        }
    }
    out
}

/// Record that `device` names each of `ranges`, all now mapped.
pub(super) fn name(held: &mut Vec<Held>, ranges: &[ReservedRange], device: DeviceAddress) {
    for range in ranges {
        match held.iter_mut().find(|h| h.range == *range) {
            Some(h) if !h.users.contains(&device) => h.users.push(device),
            Some(_) => {}
            None => held.push(Held { range: *range, users: alloc::vec![device] }),
        }
    }
}

/// `device` no longer names anything: the ranges no attached device names
/// any more leave `held`, and are returned for the caller to unmap.
pub(super) fn unname(held: &mut Vec<Held>, device: DeviceAddress) -> Vec<ReservedRange> {
    for h in held.iter_mut() {
        h.users.retain(|d| *d != device);
    }
    let freed = held.iter().filter(|h| h.users.is_empty()).map(|h| h.range).collect();
    held.retain(|h| !h.users.is_empty());
    freed
}
