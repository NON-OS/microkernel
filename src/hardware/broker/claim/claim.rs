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
use super::types::ClaimError;

// Register a claim; the granted epoch on success. The caller has already
// verified that `device_id` exists in the broker table.
pub fn claim(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    if super::firmware::is_firmware(device_id) {
        return Err(ClaimError::Firmware);
    }
    hold::admits(pid, device_id)?;
    let fresh = super::wipe::fresh(pid, device_id);
    let covers = crate::hardware::broker::bootfb::covers(device_id);
    let epoch = super::enter::enter(pid, device_id, fresh.as_ref())?;
    // Confined before it is powered, so it never runs unconfined (port_only.rs).
    let untranslated = hold::untranslated(device_id);
    let port_only = match super::port_only::confine(pid, device_id, untranslated) {
        Ok(port_only) => port_only,
        Err(refusal) => {
            super::pending::withdraw(pid, device_id, epoch);
            return Err(refusal.into());
        }
    };
    // Not mastering while the reset below waits out what is in flight (K10a).
    if !untranslated {
        super::master_off::bus_master_off(device_id);
    }
    // A port-only claim has Memory Space off too, and kept off (H-02), before
    // the reset saves the header it puts back.
    if port_only {
        super::port_only::port_only_quietly(device_id);
    }
    // In D0 before its driver maps MMIO; outside the claims lock, as it settles.
    crate::hardware::broker::power::power_on_device(device_id);
    // Powered, so its reset takes: what firmware left armed stops (K10a).
    super::reset::at_claim(device_id);
    // Its BARs put back where the move to D0 lost them; one that won't stick refuses (L62).
    if !super::restore::restore(device_id) {
        super::pending::withdraw(pid, device_id, epoch);
        return Err(ClaimError::Refused);
    }
    // Before Bus Master can go on, so no request it ever makes is no-snoop.
    super::no_snoop::snoop_every_request(device_id);
    // It starts its claim unable to master (L34; VMMDev reads past any gate), past
    // every refusal; the scanout taken then, and a refusal at the admit gives it back (L79).
    if !hold::untranslated(device_id) {
        super::master_off::bus_master_off(device_id);
    }
    if covers {
        super::scanout::take(device_id);
    }
    // Only now is anything granted against it, so a refusal above leaves no
    // map, binding or DMA behind for the next claimant's device (L65).
    let admitted = super::pending::admit(pid, device_id, epoch, fresh.as_ref(), covers);
    if covers {
        crate::kernel_core::init::framebuffer::end_takeover();
    }
    admitted.map(|()| epoch).inspect_err(|_| super::pending::withdraw(pid, device_id, epoch))
}
