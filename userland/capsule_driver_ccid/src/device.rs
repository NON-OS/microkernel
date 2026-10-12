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
//! What the driver holds: the reader bound, if any, who holds it, what it
//! remembers of the slots, and the next message's sequence number.

use nonos_usbnet::scan::Bound;
use nonos_usbnet::xhci::XhciBus;

use crate::bind::Port;
use crate::ccid::Slots;
use crate::owner::Owner;

#[derive(Default)]
pub struct Dev {
    pub bound: Option<Bound<Port>>,
    pub owner: Owner,
    pub slots: Slots,
    seq: u8,
}

impl Dev {
    pub fn bus(&self) -> Option<XhciBus> {
        self.bound.as_ref().map(|b| XhciBus { xhci: b.xhci, slot: b.slot })
    }

    pub fn port(&self) -> Option<&Port> {
        self.bound.as_ref().map(|b| &b.nic)
    }

    /// bSeq for the next command; it wraps, as the reader expects.
    pub fn next_seq(&mut self) -> u8 {
        self.seq = self.seq.wrapping_add(1);
        self.seq
    }
}
