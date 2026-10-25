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

//! Ejects in flight: one waiting for a holder to let go is tried again on
//! the tick; one whose `_EJ0` ran is done once `_STA` says the device is
//! gone, and if it never does, what was taken out is looked at again.

use super::eject::{try_eject, Refused};
use super::release::may_leave;
use super::state::{Pending, Stage};
use super::verify::{line, say_wait, verify, TAKE_OUT_MS, VERIFY_MS, VERIFY_STEP_MS};
use crate::boot::Acpi;
use crate::sys::clock::now_ms;
use alloc::vec::Vec;
use nonos_aml::hotplug::Released;
use nonos_aml::namespace::Id;

const RETRY_MS: u64 = 2_000;

fn attempt(acpi: &mut Acpi, id: Id, first: bool) {
    let now = now_ms();
    let verify = |until_ms| (Stage::Verify { until_ms }, now + VERIFY_STEP_MS);
    let (stage, next_ms) = match try_eject(acpi, id) {
        Ok(Released::ByFirmware) => verify(now + VERIFY_MS),
        Ok(Released::ForTheUser) => {
            line(acpi, id).str(b": removable, unlocked; may be taken out now").end();
            verify(now + TAKE_OUT_MS)
        }
        Err(r @ (Refused::Held(_) | Refused::HeldRecord(_))) => {
            if first {
                say_wait(acpi, id, r);
            }
            (Stage::Held, now + RETRY_MS)
        }
        Err(Refused::Firmware(why)) => {
            line(acpi, id).str(b": eject refused by firmware's ").str(why.method()).end();
            return;
        }
    };
    acpi.hotplug.pending.push(Pending { id, stage, next_ms });
}

pub fn request(acpi: &mut Acpi, id: Id) {
    if !may_leave(acpi, id) {
        line(acpi, id).str(b": eject asked of a device with no _EJ0 and no _RMV").end();
    } else if !acpi.hotplug.pending.iter().any(|p| p.id == id) {
        attempt(acpi, id, true);
    }
}

pub fn tick(acpi: &mut Acpi) {
    let now = now_ms();
    let (due, wait): (Vec<Pending>, Vec<Pending>) =
        acpi.hotplug.pending.drain(..).partition(|p| p.next_ms <= now);
    acpi.hotplug.pending = wait;
    for p in due {
        match p.stage {
            Stage::Held => attempt(acpi, p.id, false),
            Stage::Verify { until_ms } => verify(acpi, p.id, until_ms, now),
        }
    }
}
