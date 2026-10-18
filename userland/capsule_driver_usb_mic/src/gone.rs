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
//! A microphone gone from its port, root or virtual: its stream stopped
//! (the compositor told), its slot given back and its port free for the
//! search again.

use nonos_usbnet::say::say;
use nonos_usbnet::scan::Scanner;
use nonos_usbnet::xhci::disable_slot;

use crate::device::Dev;
use crate::stream::stop;
use crate::xhci::port_connected;

/// A host that does not answer is not taken for the device gone.
pub fn gone(d: &Dev) -> bool {
    let Some(b) = d.bound.as_ref() else { return false };
    port_connected(b.xhci, b.port) == Some(false)
}

pub fn give_back(d: &mut Dev, scanner: &mut Scanner) {
    stop(d);
    if let Some(b) = d.bound.take() {
        say(b"usb-mic", &[b"microphone unplugged, given back"]);
        disable_slot(b.xhci, b.slot);
        scanner.lost(b.xhci, b.port);
    }
}
