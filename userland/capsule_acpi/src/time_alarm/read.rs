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

//! TIME_ALARM (14): what the device answers, read-only. The body: a
//! presence byte (bit 0 capabilities, 1 time, then value, policy and wake
//! status for AC in bits 2 to 4 and for DC in bits 5 to 7), _GCP u32, _GRT's
//! 16 bytes as firmware gave them (zero when absent), then for AC and for
//! DC: _TIV, _TIP and _GWS, each u32. ENOENT without a device.

use crate::boot::{evaluator, Acpi};
use crate::server::protocol::ENOENT;
use alloc::vec::Vec;
use nonos_aml::time_alarm::{read, time_alarm_device, Timer};

fn word(b: &mut Vec<u8>, present: &mut u8, bit: u8, v: Option<u64>) {
    *present |= u8::from(v.is_some()) << bit;
    b.extend_from_slice(&(v.unwrap_or(0) as u32).to_le_bytes());
}

fn timer(b: &mut Vec<u8>, present: &mut u8, first: u8, t: &Timer) {
    word(b, present, first, t.value);
    word(b, present, first + 1, t.policy);
    word(b, present, first + 2, t.wake_status);
}

pub fn time_alarm(acpi: &mut Acpi) -> (i32, Vec<u8>) {
    let Some(dev) = time_alarm_device(&acpi.devices) else { return (ENOENT, Vec::new()) };
    let r = read(&mut acpi.aml, &mut evaluator(&mut acpi.host), dev);
    let (mut b, mut present) = (Vec::with_capacity(45), 0u8);
    b.push(0);
    word(&mut b, &mut present, 0, r.capabilities);
    present |= u8::from(r.now.is_some()) << 1;
    b.extend_from_slice(&r.now.and_then(|t| t.encode()).unwrap_or([0; 16]));
    timer(&mut b, &mut present, 2, &r.ac);
    timer(&mut b, &mut present, 5, &r.dc);
    b[0] = present;
    (0, b)
}
