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

extern crate alloc;

use alloc::vec::Vec;

use super::iova::IOVA_BASE;
use super::table::{pci_address, say, Capsule, CAPSULES};
use crate::memory::iommu::{IommuDomain, IommuError};

/// The unit is in service and would not take the device, so the claim is
/// refused rather than granted with a device that reaches all of memory.
pub(in crate::hardware::broker) struct Refused;

pub(in crate::hardware::broker) fn attach(pid: u32, device_id: u64) -> Result<(), Refused> {
    let Some(address) = pci_address(device_id) else {
        return Ok(());
    };
    if super::bypass::bypasses_translation(device_id) {
        say(b"unconfined: virtio without ACCESS_PLATFORM bypasses the IOMMU", pid, address);
        return Ok(());
    }
    let mut all = CAPSULES.lock();
    let pos = match all.iter().position(|c| c.pid == pid) {
        Some(i) => i,
        None => match IommuDomain::allocate() {
            Ok(domain) => {
                all.push(Capsule { pid, domain, devices: Vec::new(), next_iova: IOVA_BASE });
                all.len() - 1
            }
            // The posture on most hardware: said per claim, not only at boot.
            Err(IommuError::NotInitialized | IommuError::NotSupported) => {
                say(b"unconfined: no remapping unit in service, reaches all memory", pid, address);
                return Ok(());
            }
            Err(_) => {
                say(b"refused: no domain left", pid, address);
                return Err(Refused);
            }
        },
    };
    // Out of the identity domain first. Between the two writes the device has
    // no context entry, which denies it: the safe side to be on.
    let _ = all[pos].domain.detach_device(address);
    if all[pos].domain.attach_device(address).is_err() {
        say(b"refused: attach failed", pid, address);
        if all[pos].devices.is_empty() {
            all.remove(pos);
        }
        return Err(Refused);
    }
    all[pos].devices.push((device_id, address));
    say(b"confined to its capsule's domain", pid, address);
    Ok(())
}
