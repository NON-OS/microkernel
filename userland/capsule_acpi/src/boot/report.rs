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

//! The inventory and the interrupt routing, read once the devices are
//! initialized, and one line each on serial.

use super::eval::evaluator;
use super::report_prt::report_routes;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::string::String;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::{write_path, Id};
use nonos_aml::platform::{inventory, DeviceInfo, Route};

pub fn path(aml: &Aml, id: Id) -> String {
    let mut s = String::new();
    let _ = write_path(&aml.ns, id, &mut s);
    s
}

fn say(aml: &Aml, d: &DeviceInfo) {
    let mut l = Line::new(b"dev ").str(path(aml, d.id).as_bytes()).str(b" sta=").hex(d.sta);
    if let Some(h) = &d.hid {
        l = l.str(b" hid=").str(h.bytes());
    }
    if let Some(a) = d.adr {
        l = l.str(b" adr=").hex(a);
    }
    if let Some(u) = d.uid {
        l = l.str(b" uid=").dec(u);
    }
    l.end();
}

pub fn read(aml: &mut Aml, host: &mut CapsuleHost) -> (Vec<DeviceInfo>, Vec<(Id, Vec<Route>)>) {
    let (devices, failed) = inventory(aml, &mut evaluator(host));
    devices.iter().for_each(|d| say(aml, d));
    Line::new(b"inventory: ")
        .dec(devices.len() as u64)
        .str(b" devices, ")
        .dec(failed.len() as u64)
        .str(b" objects failed")
        .end();
    let all = report_routes(aml, host, &devices);
    (devices, all)
}
