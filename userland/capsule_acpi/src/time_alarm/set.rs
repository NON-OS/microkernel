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

//! TIME_SET (15, verb): ALARM (0, timer u8, seconds u32) arms the AC (0) or
//! DC (1) wake timer through _STV; TIME (1, 16 bytes laid out as _GRT's)
//! sets firmware's clock through _SRT. EPERM while the switch is off;
//! ENOENT without a device; EINVAL for a short request or a time or timer
//! out of range; EIO when firmware refuses or the method fails.

use super::switch::WRITES;
use crate::boot::{evaluator, Acpi};
use crate::server::protocol::{EINVAL, EIO, ENOENT, EPERM};
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::time_alarm::{set_alarm, set_time, time_alarm_device, RealTime, WriteError};

pub fn time_set(acpi: &mut Acpi, req: &[u8]) -> (i32, Vec<u8>) {
    if !WRITES {
        Line::new(b"time_set refused: the switch is off").end();
        return (EPERM, Vec::new());
    }
    let Some(dev) = time_alarm_device(&acpi.devices) else { return (ENOENT, Vec::new()) };
    let mut eval = evaluator(&mut acpi.host);
    let r = match (req.get(1), req.get(2..)) {
        (Some(0), Some(&[timer, a, b, c, d, ..])) => {
            let seconds = u32::from_le_bytes([a, b, c, d]);
            set_alarm(&mut acpi.aml, &mut eval, dev, u64::from(timer), seconds)
        }
        (Some(1), Some(t)) => match t.get(..16).and_then(RealTime::decode) {
            Some(t) => set_time(&mut acpi.aml, &mut eval, dev, &t),
            None => Err(WriteError::Invalid),
        },
        _ => Err(WriteError::Invalid),
    };
    match r {
        Ok(()) => (0, Vec::new()),
        Err(WriteError::Invalid) => (EINVAL, Vec::new()),
        Err(_) => (EIO, Vec::new()),
    }
}
