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

use super::one::attach_one;
use super::say::say;
use super::streams::attach_streams;
use super::table::pci_address;

/// The unit is in service and would not take the device, so the claim is
/// refused rather than granted with a device that reaches all of memory.
pub(in crate::hardware::broker) struct Refused;

pub(in crate::hardware::broker) fn attach(pid: u32, device_id: u64) -> Result<(), Refused> {
    let Some(address) = pci_address(device_id) else {
        // A device the firmware description names: confined through its
        // `iommus`, or refused (streams.rs).
        return match crate::hardware::broker::described::iommus(device_id) {
            Some(iommus) => attach_streams(pid, device_id, iommus),
            None => Ok(()),
        };
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
    attach_one(pid, device_id, address, false)
}
