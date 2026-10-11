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

//! Which driver plays: a USB device someone plugged in, while it says it is
//! ready, else the machine's HD Audio. Pure, for the proofs.

/// The port to play through, given whether driver.usb_audio0 reports a
/// device ready and which of the two drivers this machine registered.
pub fn choose(usb_ready: bool, hda: Option<u32>, usb: Option<u32>) -> Option<u32> {
    match (usb_ready, usb) {
        (true, Some(usb)) => Some(usb),
        _ => hda.or(usb),
    }
}
