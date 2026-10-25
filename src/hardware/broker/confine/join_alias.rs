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

//! A claim's isolation group and requester-id aliases, held to its capsule
//!. A group member or an alias in another capsule's domain reaches
//! this device past the IOMMU; every alias joins the claimer's domain with
//! the device, or the DMA it carries is denied.

extern crate alloc;

use alloc::vec::Vec;

use super::refuse::Refused;
use super::say::say;
use super::table::Capsule;
use crate::memory::iommu::DeviceAddress;

/// Each alias into the domain that now holds `device_id`, recorded under it
/// so a release takes it out. Refused, the device leaves the domain again.
pub(super) fn join_aliases(
    all: &mut Vec<Capsule>,
    pid: u32,
    device_id: u64,
    aliases: &[DeviceAddress],
) -> Result<(), Refused> {
    let Some(pos) = all.iter().position(|c| c.pid == pid) else {
        return Err(Refused);
    };
    for &alias in aliases {
        let shared = all[pos].devices.iter().any(|(_, a)| *a == alias);
        if !shared {
            let _ = all[pos].domain.detach_device(alias);
            if all[pos].domain.attach_device(alias).is_err() {
                say(b"refused: a requester-id alias did not attach");
                super::detach::detach_held(all, pid, device_id);
                return Err(Refused);
            }
        }
        all[pos].devices.push((device_id, alias));
    }
    Ok(())
}
