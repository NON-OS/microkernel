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

//! A Notify on a hotkey source. On a WMI mapper, the value names one of its
//! event blocks and _WED(value) returns the event's data (Microsoft's WMI
//! mapping). On Intel's HID event filter, 0x80 says an event is waiting,
//! and HDEM returns its code. On the virtual buttons, the value is the
//! code itself. Each is logged as it is; none is acted on until a laptop
//! has shown what it means.

use super::state::Hotkeys;
use crate::boot::evaluate;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::object::Object;
use nonos_aml::wmi::guid_text;

fn data(l: Line, o: Option<Object>) -> Line {
    match o {
        Some(Object::Integer(v)) => l.str(b" data=").hex(v),
        Some(Object::Buffer(b)) => l.str(b" bytes=").dec(b.borrow().len() as u64),
        Some(_) => l.str(b" data=(object)"),
        None => l.str(b" no data"),
    }
}

pub fn notified(h: &Hotkeys, aml: &mut Aml, host: &mut CapsuleHost, id: Id, value: u8) -> bool {
    if let Some(m) = h.wmi.iter().find(|m| m.id == id) {
        let Some(e) = m.events.iter().find(|e| e.notify_value() == Some(value)) else {
            Line::new(b"wmi notify ").hex(u64::from(value)).str(b" names no event").end();
            return true;
        };
        let wed = aml.ns.child(id, *b"_WED");
        let got =
            wed.and_then(|w| evaluate(aml, host, w, vec![Object::Integer(u64::from(value))]).ok());
        let l = Line::new(b"wmi event ").str(guid_text(&e.guid).as_bytes());
        data(l, got).end();
        return true;
    }
    if h.intel_hid.contains(&id) {
        let hdem = aml.ns.child(id, *b"HDEM");
        let got = hdem.and_then(|m| evaluate(aml, host, m, Vec::new()).ok());
        data(Line::new(b"intel hid notify ").hex(u64::from(value)), got)
            .str(b" (unverified)")
            .end();
        return true;
    }
    if h.intel_buttons.contains(&id) {
        Line::new(b"intel virtual button ").hex(u64::from(value)).str(b" (unverified)").end();
        return true;
    }
    false
}
