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

//! The claim-time side of `hold_rule`: who the caller is, and what the record
//! names.

use super::hold_platform::hold_for_platform;
use super::hold_rule::{hold_for, hold_for_firmware, untranslated as untranslated_ids, Hold};
use super::types::ClaimError;
use crate::hardware::broker::{table, BusKind, DeviceRecord};

/*
 * A bridge is never claimed (L58). Claim and release both clear its Bus
 * Master Enable, which on a bridge gates every request from below it, MSI
 * writes included, so either would stop each device behind the port, every
 * one another capsule's; and the registers after its BARs are its bus numbers
 * and windows. Refused to every claimant until a port claim that never writes
 * its Command, power or window registers exists.
 */
pub(super) fn admits(pid: u32, device_id: u64) -> Result<(), ClaimError> {
    let pci = crate::hardware::broker::pci_index::lookup(device_id);
    if pci.is_some_and(|h| !h.standard) {
        return Err(ClaimError::Refused);
    }
    let Some(hold) = rule_for(device_id) else {
        return Ok(());
    };
    match hold {
        Hold::Open => Ok(()),
        Hold::ForService(name) if serves(pid, name) => Ok(()),
        Hold::ForService(_) => Err(ClaimError::Held),
        Hold::Refused => Err(ClaimError::Refused),
    }
}

pub(super) fn untranslated(device_id: u64) -> bool {
    pci_record(device_id).is_some_and(|r| untranslated_ids(r.vendor, r.device))
}

fn rule_for(device_id: u64) -> Option<Hold> {
    let rec = table::list().into_iter().find(|r| r.device_id == device_id)?;
    match rec.bus_kind {
        k if k == BusKind::Pci as u8 => Some(hold_for(rec.vendor, rec.device)),
        k if k == BusKind::Firmware as u8 => Some(hold_for_firmware(rec.device)),
        k => hold_for_platform(k, rec.class, rec.pci_class, rec.vendor, rec.device),
    }
}

fn pci_record(device_id: u64) -> Option<DeviceRecord> {
    table::list().into_iter().find(|r| r.device_id == device_id && r.bus_kind == BusKind::Pci as u8)
}

// A thread claims for its process, which is the one the spawn registered the
// service to.
fn serves(pid: u32, name: &str) -> bool {
    let group = crate::hardware::broker::holder_of(pid);
    crate::services::registry::lookup_service(name).is_some_and(|ep| ep.pid == group)
}
