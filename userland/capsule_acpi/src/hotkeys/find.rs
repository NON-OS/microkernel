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

//! Firmware's hotkey sources among the inventory: WMI mappers (PNP0C14)
//! with the event GUIDs their _WDG publishes, and Intel's HID event filter
//! and virtual buttons (INT33D5, INT33D6). Each event is logged under the
//! name firmware gives it, a GUID or a code, and never a guessed meaning:
//! what a code stands for is the vendor's, and stays unverified until a
//! laptop's log shows it.

use super::state::{Hotkeys, Mapper};
use crate::boot::{evaluate, path};
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::object::Object;
use nonos_aml::platform::{DeviceInfo, HardwareId, PRESENT};
use nonos_aml::wmi::{guid_text, parse_wdg};

fn is(d: &DeviceInfo, id: &HardwareId) -> bool {
    d.hid.as_ref() == Some(id) || d.cids.contains(id)
}

fn events(aml: &mut Aml, host: &mut CapsuleHost, d: &DeviceInfo) -> Mapper {
    let wdg = aml.ns.child(d.id, *b"_WDG").and_then(|w| evaluate(aml, host, w, Vec::new()).ok());
    let bytes = match wdg {
        Some(Object::Buffer(b)) => b.borrow().clone(),
        _ => Vec::new(),
    };
    let events: Vec<_> =
        parse_wdg(&bytes).into_iter().filter(|b| b.notify_value().is_some()).collect();
    for e in &events {
        let l = Line::new(b"wmi ").str(path(aml, d.id).as_bytes()).str(b" event ");
        let l = l.hex(u64::from(e.id[0])).str(b" ");
        l.str(guid_text(&e.guid).as_bytes()).end();
    }
    Mapper { id: d.id, events }
}

pub fn find(devices: &[DeviceInfo], aml: &mut Aml, host: &mut CapsuleHost) -> Hotkeys {
    let mut h = Hotkeys::default();
    let (wmi, hid, vbtn) = (
        HardwareId::Eisa(*b"PNP0C14"),
        HardwareId::Text(b"INT33D5".to_vec()),
        HardwareId::Text(b"INT33D6".to_vec()),
    );
    for d in devices.iter().filter(|d| d.sta & PRESENT != 0) {
        if is(d, &wmi) {
            h.wmi.push(events(aml, host, d));
        } else if is(d, &hid) {
            h.intel_hid.push(d.id);
        } else if is(d, &vbtn) {
            h.intel_buttons.push(d.id);
        }
    }
    let l = Line::new(b"hotkeys wmi=").dec(h.wmi.len() as u64);
    let l = l.str(b" intel_hid=").dec(h.intel_hid.len() as u64);
    l.str(b" intel_buttons=").dec(h.intel_buttons.len() as u64).end();
    h
}
