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

//! What a translated device must pass before it joins a domain.

extern crate alloc;

use alloc::vec::Vec;

use super::refuse::Refused;
use super::table::say;

/// The device's isolation group, which join.rs holds to one capsule.
pub(super) fn admit(device_id: u64) -> Result<Vec<u64>, Refused> {
    super::interrupts::admit()?;
    // A virtio device without ACCESS_PLATFORM ignores the domain.
    if !super::virtio::may_confine(device_id) {
        say(b"refused: virtio without ACCESS_PLATFORM under an IOMMU");
        return Err(Refused);
    }
    // What it reaches without the IOMMU is its group.
    #[cfg(target_arch = "x86_64")]
    return super::acs::group(device_id).ok_or_else(|| {
        say(b"refused: no isolation group, behind a VMD or a looped bus");
        Refused
    });
    #[cfg(not(target_arch = "x86_64"))]
    Ok(Vec::new())
}
