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

//! A device the driver no longer has: one that stopped answering while it
//! played, or one taken out of its port while nothing played. Either way
//! its slot is disabled, the port is free for the scanner again, and the
//! audio server is told no device is ready.

use nonos_libc::Deadline;
use nonos_usbnet::say::say;
use nonos_usbnet::scan::{Bound, Scanner};
use nonos_usbnet::xhci::disable_slot;

use super::play::Play;
use crate::bind::Speaker;
use crate::xhci::port_connected;

const TAG: &[u8] = b"usb-audio";

pub fn give_back(
    bound: &mut Option<Bound<Speaker>>,
    scanner: &mut Scanner,
    play: &mut Play,
    why: &[u8],
) {
    if let Some(b) = bound.take() {
        say(TAG, &[why]);
        disable_slot(b.xhci, b.slot);
        scanner.lost(b.xhci, b.port);
    }
    play.lost();
}

/// While bound, playing or not, about once each `every_ms` (look.rs):
/// whether the device is gone from its port. Only CCS is read, never CSC, which driver.xhci0
/// clears on read and which every class driver shares.
pub fn unplugged(
    bound: &Option<Bound<Speaker>>,
    play: &Play,
    next: &mut Deadline,
    every_ms: u64,
) -> bool {
    if !super::look::look_now(bound.is_some(), play.running, next.expired()) {
        return false;
    }
    let Some(b) = bound.as_ref() else { return false };
    *next = Deadline::after_ms(every_ms);
    port_connected(b.xhci, b.port) == Some(false)
}
