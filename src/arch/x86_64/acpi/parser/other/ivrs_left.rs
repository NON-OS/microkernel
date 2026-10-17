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

//! Whether IVRS left an AMD IOMMU unread: past the eight the kernel
//! holds, past a block that does not parse, or past the length it reads. An
//! IOMMU left off translates nothing, so AMD-Vi then claims no enforcement
//! (backend/amd_capabilities.rs).

use core::sync::atomic::{AtomicBool, Ordering};

use super::ivrs_walk::MAX_AMD_IOMMUS;
use super::unit_base::AMD_WINDOW;

static LEFT_OFF: AtomicBool = AtomicBool::new(false);

pub(super) fn note(left_off: bool) {
    if left_off {
        LEFT_OFF.store(true, Ordering::Release);
    }
}

pub fn amd_units_left_off() -> bool {
    LEFT_OFF.load(Ordering::Acquire)
}

/// The bases whose registers may be mapped; any other is left off.
pub(super) fn admitted(found: &[u64]) -> heapless::Vec<u64, MAX_AMD_IOMMUS> {
    let mut kept = heapless::Vec::new();
    for &base in found {
        let ok = super::unit_base_env::admit(base, AMD_WINDOW, &kept).is_ok();
        note(!ok || kept.push(base).is_err());
    }
    kept
}
