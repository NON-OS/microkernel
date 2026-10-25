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
use super::decide::{verdict, Verdict};
use super::refuse::{Refusal, Refused};
use super::streams::attach_streams;
use super::table::pci_address;
use super::unconfined::unconfined;
use crate::memory::iommu::dev_unconfined::{OPT_IN, PRODUCTION};
use crate::memory::iommu::no_iommu::holds as posture;

/// Confine `device_id` to `pid`'s domain; whether the claim is port-only
/// (port_only.rs), for the claim to close its Memory Space.
pub(in crate::hardware::broker) fn attach(pid: u32, device_id: u64) -> Result<bool, Refusal> {
    let port_only = super::one::port_only_held(pid);
    attach_as(pid, device_id, port_only).map(|()| port_only)
}

fn attach_as(pid: u32, device_id: u64, port_only: bool) -> Result<(), Refusal> {
    let Some(address) = pci_address(device_id) else {
        // A device the firmware description names: confined through its
        // `iommus`, or refused (streams.rs).
        if let Some(iommus) = crate::hardware::broker::described::iommus(device_id) {
            return Ok(attach_streams(pid, device_id, iommus, port_only)?);
        }
        // No requester id, no bus master: map.rs refuses its DMA grants.
        return match verdict(false, false, Ok(()), port_only, (OPT_IN, PRODUCTION), posture()) {
            Verdict::Refuse => Err(Refused.into()),
            _ => Ok(()),
        };
    };
    /*
     * A device no unit in service covers is not translated, and an IOVA
     * handed to it would be used as a physical address: with only the first
     * unit programmed, the RTL8821CE behind a laptop's second unit staged its
     * firmware from whatever lived at 0x100000. Such a device is refused,
     * unless no unit is in service at all and the no-IOMMU posture holds
     * (unconfined.rs).
     */
    if !crate::memory::iommu::translates(address) {
        let v = verdict(true, false, no_unit(), port_only, (OPT_IN, PRODUCTION), posture());
        return Ok(unconfined(device_id, v)?);
    }
    let (group, aliases) = super::gates::admit(device_id)?;
    super::one::attach_one(pid, device_id, address, (false, port_only), (&group, &aliases))
}

/// A device that reaches memory past any IOMMU whatever its domain says
/// (VMMDev reads guest-physical addresses): it gets no domain, and goes
/// ahead only where an unconfined device may.
pub(in crate::hardware::broker) fn untranslated(device_id: u64) -> Result<(), Refusal> {
    // Its port reads memory whatever its Command register says: never port-only.
    let v = verdict(true, false, no_unit(), false, (OPT_IN, PRODUCTION), posture());
    Ok(unconfined(device_id, v)?)
}

/// What a device no unit translates has for a domain: none for want of a
/// unit when no unit is in service at all, the one case the posture opens.
fn no_unit() -> Result<(), bool> {
    if crate::memory::iommu::capabilities().enforcing {
        Ok(())
    } else {
        Err(true)
    }
}
