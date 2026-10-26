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

//! Step 5, the capsules: PCR 14, replayed from the log, records one event
//! for each capsule the kernel admitted. A release host has admitted at
//! least one, and every one on its STARK proof under the vendor's root. The
//! first that was not is named: a development image's path alone, a root a
//! person enrolled, or a local build.

use alloc::vec::Vec;

use crate::log::{Admitted, Replayed};
use crate::{CapsuleFault, Refusal};

pub fn capsules(r: &Replayed) -> Result<(), Refusal> {
    if r.capsules.is_empty() {
        return Err(Refusal::Pcr14(CapsuleFault::None));
    }
    match r.capsules.iter().map(|c| c.admitted).find(|a| *a != Admitted::Vendor) {
        None | Some(Admitted::Vendor) => Ok(()),
        Some(Admitted::PathOnly) => Err(Refusal::Pcr14(CapsuleFault::PathOnly)),
        Some(Admitted::Developer) => Err(Refusal::Pcr14(CapsuleFault::Developer)),
        Some(Admitted::LocalBuild) => Err(Refusal::Pcr14(CapsuleFault::LocalBuild)),
    }
}

/// Every capsule a release refuses: its place in PCR 14, and its kind.
pub fn not_vendor(r: &Replayed) -> Vec<(usize, Admitted)> {
    let capsules = r.capsules.iter().map(|c| c.admitted).enumerate();
    capsules.filter(|(_, a)| *a != Admitted::Vendor).collect()
}
