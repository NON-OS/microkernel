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

use super::hold;
use super::state::{claims, next_epoch};
use super::types::{Claim, ClaimError};

// Register a claim. Returns the granted epoch on success. The caller
// must have already verified that `device_id` exists in the broker
// table.
pub fn claim(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    hold::admits(pid, device_id)?;
    let epoch = {
        let mut claims = claims();
        if claims.iter().any(|c| c.device_id == device_id) {
            return Err(ClaimError::AlreadyClaimed);
        }
        let epoch = next_epoch();
        claims.push(Claim::making(pid, device_id, epoch));
        epoch
    };
    // The kernel reads a virtio-rng for boot entropy until its capsule
    // claims it: reset and let go before the device changes domain (L8).
    if let Some(handle) = crate::hardware::broker::pci_index::lookup(device_id) {
        crate::drivers::virtio_rng::hand_over(handle.address);
    }
    // Confined before it is powered, so the device never runs unconfined.
    // VMMDev is the exception: it reads guest-physical addresses past any
    // IOMMU, so it keeps them, and its grants are counted unconfined.
    if hold::untranslated(device_id) {
        crate::log::info!("[BROKER] pid={} VMMDev: untranslated, its grants unconfined", pid);
    } else if crate::hardware::broker::confine::attach(pid, device_id).is_err() {
        super::pending::withdraw(pid, device_id, epoch);
        return Err(ClaimError::Unconfined);
    }
    // Bring the device to power state D0 before its driver maps MMIO. Done
    // outside the claims lock: it touches config space and settles for a moment.
    crate::hardware::broker::power::power_on_device(device_id);
    // Its BARs read back against enumeration's, and put back where the move
    // to D0 lost them; one that will not stick refuses the claim (L62).
    if !super::restore::restore(device_id) {
        super::pending::withdraw(pid, device_id, epoch);
        return Err(ClaimError::Refused);
    }
    // Before the driver can turn bus mastering on, so no request it ever
    // makes is no-snoop.
    super::no_snoop::snoop_every_request(device_id);
    // VMMDev reads guest memory past the IOMMU and any Bus Master gate, so
    // only a translated device starts its claim unable to master (L34).
    if !hold::untranslated(device_id) {
        super::master_off::bus_master_off(device_id);
    }
    // Only now is anything granted against it, so a refusal above leaves no
    // map, binding or DMA behind for the next claimant's device (L65).
    if !super::pending::admit(pid, device_id, epoch) {
        super::pending::withdraw(pid, device_id, epoch);
        return Err(ClaimError::NotClaimed);
    }
    Ok(epoch)
}
