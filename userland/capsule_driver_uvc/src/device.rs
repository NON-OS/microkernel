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
//! What the driver holds: the camera bound, if any, the stream running on
//! it and the frame being put together, and who the stream is for.

use nonos_usbnet::scan::Bound;
use nonos_usbnet::xhci::XhciBus;
use nonos_uvc::Assembler;

use crate::bind::Cam;
use crate::gate::Session;
use crate::start::Stream;

#[derive(Default)]
pub struct Dev {
    pub bound: Option<Bound<Cam>>,
    pub stream: Option<(Stream, Assembler)>,
    pub session: Session,
    /// Batches in a row that came back in error.
    pub losses: u8,
}

impl Dev {
    pub fn bus(&self) -> Option<XhciBus> {
        self.bound.as_ref().map(|b| XhciBus { xhci: b.xhci, slot: b.slot })
    }
}
