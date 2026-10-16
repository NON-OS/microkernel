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

//! DEVICE_POWER's reply: the state after, u8 (0xFF unknown), whether the
//! device has power control, u8, its path's length u8 and bytes; for
//! verb 4 only, its six sleep facts (`SleepFacts::bytes`: _S0W, _S3D,
//! _S4D, _S3W, _S4W, then _PRW's sleeping state, 0xFF where it has none),
//! read and never armed; then, only to the holder that armed its wake, 1
//! u8 and the wakes counted since, u32.

use super::moves::current;
use super::wake_sweep::owns;
use crate::boot::{evaluator, path, Acpi};
use alloc::vec::Vec;
use nonos_aml::device_power::{power_managed, sleep_facts};
use nonos_aml::namespace::Id;

const UNKNOWN: u8 = 0xFF;

pub fn reply(acpi: &mut Acpi, dev: Id, sender: u32, facts: bool) -> Vec<u8> {
    let now = current(acpi, dev).map_or(UNKNOWN, |s| s as u8);
    let mut v = alloc::vec![now, power_managed(&acpi.aml, dev) as u8];
    let p = path(&acpi.aml, dev);
    let p = &p.as_bytes()[..p.len().min(200)];
    v.push(p.len() as u8);
    v.extend_from_slice(p);
    if facts {
        let f = sleep_facts(&mut acpi.aml, &mut evaluator(&mut acpi.host), dev);
        v.extend_from_slice(&f.bytes());
    }
    if owns(&acpi.hidden.wake.owners, dev, sender) {
        v.push(1);
        v.extend_from_slice(&acpi.hidden.wake.rw.wakes(dev).unwrap_or(0).to_le_bytes());
    }
    v
}
