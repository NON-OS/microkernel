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

//! The zones read as they fall due, and on Notify (ACPI 6.5, 11.4 and
//! 5.6.6): 0x80, the temperature changed; 0x81, the trip points did;
//! 0x82, the zone's device lists did. A Notify on a fan (0x80, it runs
//! slower than asked) is logged. A zone with a trip unread is read afresh
//! at its next reading, so one cut at start is not missing for good.

use super::apply::apply;
use super::find::refresh;
use super::limits::send_limits;
use super::passive::step_zone;
use super::read::read_zone;
use super::report::send;
use super::state::Cooling;
use crate::boot::path;
use crate::host::CapsuleHost;
use crate::sys::clock::now_ms;
use crate::sys::log::Line;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::thermal::UNREAD_TRIPS;

pub fn tick(c: &mut Cooling, aml: &mut Aml, host: &mut CapsuleHost) {
    let now = now_ms();
    let mut read = false;
    let due = |z: &super::state::Zone| now >= z.next_ms;
    for z in 0..c.zones.len() {
        if due(&c.zones[z]) && c.zones[z].trips.faults & UNREAD_TRIPS != 0 {
            refresh(c, aml, host, z);
        }
    }
    for z in c.zones.iter_mut().filter(|z| due(z)) {
        read_zone(z, aml, host, now);
        step_zone(z, now);
        read = true;
    }
    if read {
        apply(c, aml, host);
    }
    send(c, now);
    send_limits(c);
}

pub fn notified(c: &mut Cooling, aml: &mut Aml, host: &mut CapsuleHost, id: Id, value: u8) -> bool {
    if c.fans.iter().any(|f| f.id == id) {
        Line::new(b"fan ")
            .str(path(aml, id).as_bytes())
            .str(b" notify ")
            .hex(u64::from(value))
            .end();
        return true;
    }
    let Some(z) = c.zones.iter().position(|z| z.id == id) else { return false };
    if value == 0x81 || value == 0x82 {
        refresh(c, aml, host, z);
    }
    c.zones[z].next_ms = 0;
    tick(c, aml, host);
    true
}
