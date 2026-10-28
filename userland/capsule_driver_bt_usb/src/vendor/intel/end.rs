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

//! The end of every setup: Intel's event mask (0xfc52), its vendor events
//! on and no debug ones (`btintel_set_event_mask`), which the controller
//! works without, then Reset.

use super::wire::{Say, Wire};
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::HciError;

const EVENT_MASK: u16 = 0xfc52;
const RESET: u16 = 0x0c03;

pub fn end<P: UsbPipes>(w: &mut Wire<P>) -> Result<(), HciError> {
    match w.command(EVENT_MASK, &[0x87, 0x0c, 0, 0, 0, 0, 0, 0]) {
        Ok(_) | Err(HciError::Status(..)) => w.command(RESET, &[]).map(drop),
        Err(e) => Err(e),
    }
}

pub fn say(to: Say, what: &[u8], stem: &str) {
    to(&[b"intel: ", what, stem.as_bytes()]);
}
