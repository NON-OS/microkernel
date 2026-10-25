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
//! What the driver holds: the line bound, if any, who holds it, what was
//! read and not yet taken, and the line state the device last told.

use nonos_usbnet::scan::Bound;
use nonos_usbnet::xhci::XhciBus;

use crate::acm::{Owner, Rx, State};
use crate::bind::Port;

#[derive(Default)]
pub struct Dev {
    pub bound: Option<Bound<Port>>,
    pub owner: Owner,
    pub rx: Rx,
    pub state: State,
}

impl Dev {
    pub fn bus(&self) -> Option<XhciBus> {
        self.bound.as_ref().map(|b| XhciBus { xhci: b.xhci, slot: b.slot })
    }

    pub fn port(&self) -> Option<&Port> {
        self.bound.as_ref().map(|b| &b.nic)
    }
}
