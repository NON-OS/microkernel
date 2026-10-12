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

//! Arming a bank's pins from what firmware says: every `_AEI` pin, owned
//! by capsule_acpi; every serial-bus device's GpioInt on this controller
//! (matched by `_UID`), owned by the PCI function of the device's bus
//! controller, whose holder alone hears it. A pin the model refuses is
//! said and left unarmed.

use crate::acpi::serial;
use crate::aei::aei_list;
use crate::bank::Bank;
use crate::say::say;
use nonos_gpio::{Owner, Trigger};

const GPIO_INT: u8 = 1 << 3;
const EDGE: u8 = 1 << 5;
const ACTIVE_LOW: u8 = 1 << 6;
/// Serial-bus devices looked at, at most.
const MAX_SERIAL: u32 = 64;

pub fn arm(bank: &mut Bank) {
    for (pin, flags) in aei_list(bank.record.device_id) {
        let Some(t) = Trigger::from_flags(flags) else { continue };
        match bank.router.bind(&mut bank.ctl, pin, Owner::Aei, t) {
            Ok(()) if !t.edge => bank.level_aei.push(pin),
            Ok(()) => {}
            Err(e) => say(b"aei pin not armed", bank.uid, pin, &e),
        }
    }
    let mut index = 0;
    while index < MAX_SERIAL {
        let Ok(r) = serial(index) else { break };
        index += 1;
        let Some((key, pin, t)) = serial_pin(&r, bank.uid) else { continue };
        if let Err(e) = bank.router.bind(&mut bank.ctl, pin, Owner::Device(key), t) {
            say(b"device pin not armed", bank.uid, pin, &e);
        }
        if index >= u32::from_le_bytes([r[0], r[1], r[2], r[3]]) {
            break;
        }
    }
}

/// A SERIAL record's GpioInt on controller `uid`: the bus controller's PCI
/// function (segment 0, bus 0, from its `_ADR`), the pin, its trigger.
fn serial_pin(r: &[u8], uid: u16) -> Option<(u64, u16, Trigger)> {
    let flags = *r.get(5)?;
    let field =
        |at: usize| r.get(at..at + 8).map(|b| u64::from_le_bytes(b.try_into().unwrap_or([0; 8])));
    let (adr, gpio_uid) = (field(18)?, field(34)?);
    if flags & GPIO_INT == 0 || gpio_uid != u64::from(uid) || adr == u64::MAX {
        return None;
    }
    let pin = u16::try_from(u32::from_le_bytes(r.get(14..18)?.try_into().ok()?)).ok()?;
    let (dev, func) = ((adr >> 16) & 0x1F, adr & 7);
    let polarity = if flags & ACTIVE_LOW != 0 { 1 << 1 } else { 0 };
    let t = Trigger::from_flags(u16::from(flags & EDGE != 0) | polarity)?;
    Some((dev << 3 | func, pin, t))
}
