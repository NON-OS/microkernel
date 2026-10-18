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

//! Finding a Bluetooth controller on driver.xhci0's ports. The port book
//! is nonos_usbnet's: a port is tried a few times, a device of another
//! kind is not tried again until it is replugged, and every port is
//! looked at afresh once a minute.

use nonos_libc::Deadline;
use nonos_usbnet::scan::Book;
use nonos_usbnet::xhci::{connected_ports, disable_slot, enable_slot, lookup, XhciBus};

use super::desk::Desk;
use super::probe::{probe, Probe};
use crate::usb::Bound;

const RETRY_ALL_MS: u64 = 60_000;

pub struct Finder {
    xhci: Option<u32>,
    book: Book,
    retry_all: Deadline,
}

impl Finder {
    pub fn new() -> Self {
        let retry_all = Deadline::after_ms(RETRY_ALL_MS);
        Self { xhci: None, book: Book::default(), retry_all }
    }

    /// A bound controller and its port, when one is plugged in.
    pub fn pass(&mut self, desk: &Desk) -> Option<(Bound, u8)> {
        let xhci = match self.xhci {
            Some(x) => x,
            None => *self.xhci.insert(lookup()?),
        };
        if self.retry_all.expired() {
            self.book.retry_all();
            self.retry_all = Deadline::after_ms(RETRY_ALL_MS);
        }
        for port in self.book.plan(&connected_ports(xhci).ok()?) {
            let Ok(slot) = enable_slot(xhci) else { continue };
            match probe(XhciBus { xhci, slot }, port, desk) {
                Probe::Ours(bound) => return Some((bound, port)),
                Probe::NotOurs => self.book.not_ours(port),
                Probe::Busy => {}
                Probe::Failed => self.book.failed(port),
            }
            disable_slot(xhci, slot);
        }
        None
    }

    /// A controller that stopped working counts a failure on its port, so
    /// one that keeps failing is left alone.
    pub fn lost(&mut self, port: u8) {
        self.book.failed(port);
    }
}
