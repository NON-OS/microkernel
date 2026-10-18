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
//! ready, else the machine's own sink. The kernel registers each driver's
//! name when it spawns it, before this capsule, and drops it when the driver
//! finds no controller: HD Audio on most machines, AC'97 in the virtual
//! machines that still present one. Pure, for the proofs.

/// The machine's own PCM sink drivers, in the order they are preferred.
pub const SINKS: [&str; 2] = ["driver.hda0", "driver.ac97_0"];
/// The log line for each of `SINKS` when it is the one played through.
pub const SINK_LINES: [&str; 2] =
    ["[AUDIO] output driver.hda0\n", "[AUDIO] output driver.ac97_0\n"];

/// Which of `SINKS` is the machine's own. `found[i]`: `SINKS[i]` is
/// registered. One registered is taken as it is. With both, `ready(i)` asks
/// each in turn whether this machine plays through it and the first that
/// does is taken, so an HD Audio controller that cannot play (HDMI only, no
/// codec) does not hide a working AC'97; with neither ready the first keeps
/// its say on why not. `ready` is asked only when more than one is found.
pub fn machine(found: [bool; 2], mut ready: impl FnMut(usize) -> bool) -> Option<usize> {
    let first = found.iter().position(|&f| f)?;
    if found.iter().filter(|&&f| f).count() == 1 {
        return Some(first);
    }
    (0..found.len()).find(|&i| found[i] && ready(i)).or(Some(first))
}

/// The port to play through, given whether driver.usb_audio0 reports a
/// device ready, the machine's own sink (`machine`) and the USB driver, each
/// as this machine registered it.
pub fn choose(usb_ready: bool, own: Option<u32>, usb: Option<u32>) -> Option<u32> {
    match (usb_ready, usb) {
        (true, Some(usb)) => Some(usb),
        _ => own.or(usb),
    }
}
