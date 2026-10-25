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

//! A claim refused, said once per claim.

extern crate alloc;

use alloc::vec::Vec;

use super::decide::Verdict;
use super::say::say;
use super::table::Capsule;

/// No unit confines the device, or the one in service would not take it, so
/// the claim is refused rather than granted with a device that reaches all
/// of memory (decide.rs).
pub(in crate::hardware::broker) struct Refused;

/// Why a claim's confinement was refused, for its errno (K11b).
pub(in crate::hardware::broker) enum Refusal {
    Refused,
    /// Another capsule holds what the device reaches past the IOMMU.
    GroupHeld,
}

impl From<Refused> for Refusal {
    fn from(_: Refused) -> Self {
        Refusal::Refused
    }
}

/// A PCI device with no domain of its own: refused, but claimed port-only
/// (decide.rs, H-02b), or under the opt-in or the no-IOMMU posture (open.rs).
pub(super) fn without_domain(v: Verdict) -> Result<(), Refused> {
    match v {
        Verdict::NoDma => {
            say(b"port-only with no remapping unit over it: Bus Master off, no DMA grant");
            return Ok(());
        }
        Verdict::Unconfined => {
            say(super::open::unconfined_words());
            return Ok(());
        }
        Verdict::Confine | Verdict::Refuse => {}
    }
    say(b"refused: no remapping unit confines it");
    Err(Refused)
}

/// Say why, and drop the capsule's entry if this was to be its first device.
pub(super) fn refuse(
    all: &mut Vec<Capsule>,
    pos: usize,
    why: &[u8],
    refusal: Refusal,
) -> Result<(), Refusal> {
    say(why);
    if all[pos].devices.is_empty() {
        all.remove(pos);
    }
    Err(refusal)
}
