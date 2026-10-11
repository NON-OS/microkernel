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

use super::decide::{verdict, Verdict};
use super::iova::IOVA_BASE;
use super::posture::unconfined_allowed;
use super::refuse::{without_domain, Refused};
use super::table::{pci_address, say, Capsule, CAPSULES};
use crate::memory::iommu::dev_unconfined::{OPT_IN, PRODUCTION};
use crate::memory::iommu::IommuDomain;

pub(in crate::hardware::broker) fn attach(pid: u32, device_id: u64) -> Result<(), Refused> {
    let Some(address) = pci_address(device_id) else {
        // No requester id, no bus master: map.rs refuses its DMA grants.
        return match verdict(false, false, Ok(()), OPT_IN, PRODUCTION) {
            Verdict::Refuse => Err(Refused),
            _ => Ok(()),
        };
    };
    /*
     * A device no unit in service covers is not translated, and an IOVA
     * handed to it would be used as a physical address: with only the first
     * unit programmed, the RTL8821CE behind a laptop's second unit staged its
     * firmware from whatever lived at 0x100000. Such a device is refused.
     */
    if !crate::memory::iommu::translates(address) {
        return without_domain(pid, address, verdict(true, false, Ok(()), OPT_IN, PRODUCTION));
    }
    // A virtio device without ACCESS_PLATFORM ignores the domain.
    if !super::virtio::may_confine(device_id) {
        say(b"refused: virtio without ACCESS_PLATFORM under an IOMMU", pid, address);
        return Err(Refused);
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
             * No unit in service is the same whether none was found, none is
             * up yet, or the one found (AMD-Vi) has no backend here: refused.
             * A unit in service with no domain left: refused as well.
             */
            Err(e) => {
                let v = verdict(true, true, Err(unconfined_allowed(e)), OPT_IN, PRODUCTION);
                return without_domain(pid, address, v);
            }
        },
    };
    super::join::join(&mut all, pos, pid, device_id, address)
}
