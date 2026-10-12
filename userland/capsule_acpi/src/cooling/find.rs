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

//! The thermal zones and the fans their _ALx lists name (ACPI 6.5, 11.4.2
//! and 11.3). A fan named by several zones, or levels, is one fan.

use super::drive::drive_of;
use super::state::{Cooling, Fan, Zone};
use crate::boot::evaluator;
use crate::host::CapsuleHost;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::thermal::{active_devices, trips, zones};

fn fan(c: &mut Cooling, aml: &mut Aml, host: &mut CapsuleHost, dev: Id) -> usize {
    if let Some(i) = c.fans.iter().position(|f| f.id == dev) {
        return i;
    }
    let drive = drive_of(aml, host, dev);
    c.fans.push(Fan { id: dev, drive, on: None });
    c.fans.len() - 1
}

/// Reads a zone's trip points and its fans afresh: at start, and when the
/// zone says they changed (Notify 0x81 and 0x82).
pub fn refresh(c: &mut Cooling, aml: &mut Aml, host: &mut CapsuleHost, z: usize) {
    let tz = c.zones[z].id;
    let t = trips(aml, &mut evaluator(host), tz);
    let mut lv = Vec::new();
    for x in 0..t.active.len() {
        let devs = active_devices(aml, &mut evaluator(host), tz, x);
        lv.push(devs.into_iter().map(|d| fan(c, aml, host, d)).collect());
    }
    (c.zones[z].trips, c.zones[z].levels) = (t, lv);
}

pub fn find(aml: &mut Aml, host: &mut CapsuleHost) -> Cooling {
    let mut c = Cooling::default();
    for id in zones(aml) {
        c.zones.push(Zone {
            id,
            trips: Default::default(),
            levels: Vec::new(),
            verdict: Default::default(),
            temp: None,
            next_ms: 0,
            critical_at: None,
            critical: false,
        });
        let z = c.zones.len() - 1;
        refresh(&mut c, aml, host, z);
    }
    c
}
