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
use super::posture::unconfined_allowed;
use super::table::{pci_address, say, Capsule, CAPSULES};
use crate::memory::iommu::IommuDomain;

/// The unit is in service and would not take the device, so the claim is
/// refused rather than granted with a device that reaches all of memory.
pub(in crate::hardware::broker) struct Refused;

pub(in crate::hardware::broker) fn attach(pid: u32, device_id: u64) -> Result<(), Refused> {
    let Some(address) = pci_address(device_id) else {
        return Ok(());
    };
    /*
     * A device no unit in service covers is not translated, and an IOVA
     * handed to it would be used as a physical address: with only the first
     * unit programmed, the RTL8821CE behind a laptop's second unit staged its
     * firmware from whatever lived at 0x100000. Such a device stays on
     * physical addresses, said and counted.
     */
    if !crate::memory::iommu::translates(address) {
        say(
            b"unconfined: no remapping unit in service covers it, reaches all memory",
            pid,
            address,
        );
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
            /*
             * The posture on most hardware: said per claim, not only at boot.
             * No unit in service is the same whether none was found, none is
             * up yet, or the one found (AMD-Vi) has no backend here.
             */
            Err(e) if unconfined_allowed(e) => {
                say(b"unconfined: no remapping unit in service, reaches all memory", pid, address);
                return Ok(());
            }
            Err(_) => {
                say(b"refused: no domain left", pid, address);
                return Err(Refused);
            }
        },
    };
    // Drives behind one VMD share its requester id. One capsule may hold
    // several of them in its domain; a second capsule cannot take the id
    // without pulling it out from under the first.
    if all.iter().enumerate().any(|(i, c)| i != pos && c.devices.iter().any(|(_, a)| *a == address))
    {
        say(b"refused: requester id held by another capsule", pid, address);
        if all[pos].devices.is_empty() {
            all.remove(pos);
        }
        return Err(Refused);
    }
    if all[pos].devices.iter().any(|(_, a)| *a == address) {
        all[pos].devices.push((device_id, address));
        say(b"confined to its capsule's domain (shared requester id)", pid, address);
        return Ok(());
    }
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
