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
//! The line read while a client holds it: the bulk IN pipe while a whole
//! read fits what is kept, and the notification endpoint for the line
//! state. Nothing is read while no one holds it, so the device keeps
//! what it has until someone listens.

use nonos_usbnet::xhci::E_PIPE;
use nonos_usbnet::Bus;

use crate::acm::serial_state;
use crate::device::Dev;
use crate::notify::{poll, NOTICE_MAX};

/// One read: a page, a whole number of packets at every speed.
const READ: usize = 4096;

pub fn pump(d: &mut Dev) {
    let (Some(mut bus), Some(notify)) = (d.bus(), d.port().map(|p| p.notify)) else { return };
    if d.owner.pid.is_none() {
        return;
    }
    if d.rx.fits(READ) {
        let mut buf = [0u8; READ];
        match bus.bulk_in_poll(&mut buf) {
            Ok(Some(n)) => d.rx.put(&buf[..n.min(READ)]),
            // A halted pipe is cleared; the next look reads again.
            Err(E_PIPE) => {
                let _ = bus.reset_bulk(true);
            }
            _ => {}
        }
    }
    if let Some(dci) = notify {
        let mut n = [0u8; NOTICE_MAX];
        if let Ok(Some(len)) = poll(bus.xhci, bus.slot, dci, &mut n) {
            if let Some(s) = serial_state(&n[..len]) {
                d.state.seen(s);
            }
        }
    }
}
