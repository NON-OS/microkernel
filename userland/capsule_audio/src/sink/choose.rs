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

//! Which output driver the server plays through. The kernel registers each
//! fixed sink's name when it spawns the driver, before this capsule, and
//! drops it when the driver finds no controller: HD Audio on most machines,
//! AC'97 in the virtual machines that still present one. One registered
//! sink is taken as it is. With both, the first that says it plays is
//! taken, so an HD Audio controller that cannot play (HDMI only, no codec)
//! does not hide a working AC'97; with neither ready the first keeps its
//! say on why not. A USB device someone plugged in is played through over
//! that sink while it says it is ready (`with_usb`). Pure, for the proofs.

/// The fixed PCM sink drivers, in the order they are preferred.
pub const SINKS: [&str; 2] = ["driver.hda0", "driver.ac97_0"];
/// The log line for each of `SINKS` when it is the one played through.
pub const SINK_LINES: [&str; 2] =
    ["[AUDIO] output driver.hda0\n", "[AUDIO] output driver.ac97_0\n"];

/// `found[i]`: `SINKS[i]` is registered. `ready(i)` asks it whether this
/// machine plays through it, and is asked only when more than one is found.
pub fn choose(found: [bool; 2], mut ready: impl FnMut(usize) -> bool) -> Option<usize> {
    let first = found.iter().position(|&f| f)?;
    if found.iter().filter(|&&f| f).count() == 1 {
        return Some(first);
    }
    (0..found.len()).find(|&i| found[i] && ready(i)).or(Some(first))
}

/// The port to play through, given whether driver.usb_audio0 reports a
/// device ready, the fixed sink `choose` took, and the USB driver's port.
/// With no fixed sink the USB driver is kept, so a device can still come.
pub fn with_usb(usb_ready: bool, fixed: Option<u32>, usb: Option<u32>) -> Option<u32> {
    match (usb_ready, usb) {
        (true, Some(usb)) => Some(usb),
        _ => fixed.or(usb),
    }
}
