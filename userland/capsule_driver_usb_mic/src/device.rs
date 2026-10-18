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
//! What the driver holds: the microphone bound, if any, whether its stream
//! runs, what was captured and not yet read, and who the stream is for.

use nonos_usbnet::scan::Bound;
use nonos_usbnet::xhci::XhciBus;

use crate::bind::Mic;
use crate::gate::Session;
use crate::ring::Ring;

#[derive(Default)]
pub struct Dev {
    pub bound: Option<Bound<Mic>>,
    pub streaming: bool,
    pub ring: Ring,
    pub session: Session,
    /// Batches in a row that came back in error.
    pub losses: u8,
}

impl Dev {
    pub fn bus(&self) -> Option<XhciBus> {
        self.bound.as_ref().map(|b| XhciBus { xhci: b.xhci, slot: b.slot })
    }
}
