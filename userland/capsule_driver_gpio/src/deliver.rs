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

//! Where a pending pin goes: an `_AEI` pin's event to capsule_acpi at once
//! (a level pin masked until its method has run); a device's pin into the
//! queue for its holder; a storming pin masked and said once.

use crate::aei::aei_signal;
use crate::bank::Bank;
use crate::queue::Queue;
use crate::say::say;
use nonos_gpio::{Controller, Delivery};

pub fn deliver(b: &mut Bank, queue: &mut Queue, d: Delivery) {
    match d {
        Delivery::Acpi { pin } => {
            let level = b.level_aei.contains(&pin);
            if level {
                let _ = b.ctl.mask(pin, true);
            }
            let status = aei_signal(b.record.device_id, pin);
            if level {
                let _ = b.ctl.mask(pin, false);
            }
            if status != 0 {
                say(b"aei method did not run", b.uid, pin, &status);
            }
        }
        Delivery::Holder { device, pin } => queue.push(device, b.uid, pin),
        Delivery::Stormed { pin, owner } => say(b"pin masked for storming", b.uid, pin, &owner),
    }
}
