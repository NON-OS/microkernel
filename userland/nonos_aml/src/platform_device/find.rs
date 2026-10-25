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

//! Each present and enabled device whose `_HID`, or failing that one of
//! its `_CID`s, is on the list, with the windows and the interrupt its
//! evaluated `_CRS` gives. Enabled because a device that is present but
//! not enabled decodes none of its resources (ACPI 6.5, 6.3.7).

use super::device::{Found, PlatformDevice, Skip};
use super::known::Known;
use super::windows::windows;
use crate::load::Aml;
use crate::object::Object;
use crate::platform::{child, interrupt_entries, DeviceInfo, Eval, ENABLED, PRESENT};
use alloc::vec::Vec;

fn matching<'k>(d: &DeviceInfo, list: &[Known<'k>]) -> Option<Known<'k>> {
    let ids = d.hid.iter().chain(&d.cids);
    ids.map(|h| h.bytes()).find_map(|b| list.iter().find(|k| k.id == b).copied())
}

pub fn platform_devices(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    devices: &[DeviceInfo],
    list: &[Known<'_>],
) -> Found {
    let mut found = Found::default();
    let usable = |d: &&DeviceInfo| d.sta & (PRESENT | ENABLED) == PRESENT | ENABLED;
    for d in devices.iter().filter(usable) {
        let Some(k) = matching(d, list) else { continue };
        let crs = match child(aml, eval, d.id, *b"_CRS", Vec::new()) {
            Some(Ok(Object::Buffer(b))) => b.borrow().clone(),
            _ => {
                found.skipped.push((d.id, Skip::NoTemplate));
                continue;
            }
        };
        let w = windows(&crs);
        if w.is_empty() {
            found.skipped.push((d.id, Skip::NoWindow));
            continue;
        }
        let (class, matched, uid) = (k.class, k.id.to_vec(), d.uid.unwrap_or(0));
        let irq = interrupt_entries(&crs).first().copied();
        found.devices.push(PlatformDevice { id: d.id, class, matched, uid, windows: w, irq });
    }
    found
}
