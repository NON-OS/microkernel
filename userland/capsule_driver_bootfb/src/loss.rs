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
//! The scanout lost to a GPU driver (G-80, L79): when a driver's claim
//! covers the firmware framebuffer, the kernel ends bootfb's claim and
//! points every page of its mapping at one zeroed sink. Nothing faults, so
//! bootfb asks: `mk_boot_framebuffer` answering anything but its 24 bytes
//! means the scanout is gone, for the boot. It is asked before every frame
//! served and on every wake (at least every 500 ms). On loss bootfb stops
//! drawing, releases the old claim once (ignoring -ENODEV), never maps or
//! claims again, and the compositor is told once that its display left;
//! from then on the one output answers LOST.

use super::display::BootFb;

impl BootFb {
    /// Whether the scanout is still this driver's; the first time it is
    /// not, the claim is let go of.
    pub fn poll(&mut self) {
        if self.gone || (self.holds)(self.device_id) {
            return;
        }
        self.gone = true;
        (self.release)(self.device_id);
    }

    /// Polled; whether the compositor is now to be told the display left.
    pub fn tell_due(&mut self) -> bool {
        self.poll();
        self.gone && !core::mem::replace(&mut self.told, true)
    }
}
