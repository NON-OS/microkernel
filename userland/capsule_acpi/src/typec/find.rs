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

//! The UCSI device: present, _HID USBC000 or a _CID PNP0CA0, a _CRS memory
//! window at least as large as a revision 1.x mailbox, and a _DSM with
//! both signals. The session starts here, or the reason it did not is
//! logged and there is no Type-C on the power wire.

use super::dsm::query;
use super::ppm::AcpiPpm;
use super::state::TypeC;
use crate::boot::evaluator;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::object::Object;
use nonos_aml::platform::{child, DeviceInfo, PRESENT};
use nonos_aml::platform_device::windows;
use nonos_ucsi::layout::MAILBOX_MIN;
use nonos_ucsi::session::Session;

fn is_ucsi(d: &DeviceInfo) -> bool {
    d.hid.iter().chain(&d.cids).any(|h| matches!(h.bytes(), b"USBC000" | b"PNP0CA0"))
}

fn no(why: &[u8]) -> Option<TypeC> {
    Line::new(b"ucsi: ").str(why).end();
    None
}

pub fn find(devices: &[DeviceInfo], aml: &mut Aml, host: &mut CapsuleHost) -> Option<TypeC> {
    let d = devices.iter().find(|d| d.sta & PRESENT != 0 && is_ucsi(d))?;
    let crs = match child(aml, &mut evaluator(host), d.id, *b"_CRS", Vec::new()) {
        Some(Ok(Object::Buffer(b))) => b.borrow().clone(),
        _ => return no(b"no _CRS"),
    };
    let Some(w) = windows(&crs).into_iter().find(|w| w.len >= MAILBOX_MIN) else {
        return no(b"no mailbox window in _CRS");
    };
    let Some(revision) = query(aml, host, d.id) else { return no(b"_DSM lacks read or write") };
    let (dev, mailbox) = (d.id, (w.base, w.len));
    let mut ppm = AcpiPpm { aml, host, dev, mailbox, revision };
    match Session::start(&mut ppm) {
        Ok(session) => {
            let t = TypeC { dev, mailbox, revision, session };
            super::say::started(&t);
            Some(t)
        }
        Err(_) => no(b"the PPM did not reset or answer; not started"),
    }
}
