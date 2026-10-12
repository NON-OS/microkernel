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

//! SURFACE: the current primary, made on first ask at the mode the device
//! shows, and shared with the asker. A new asker becomes the owner, and a
//! primary still waiting for RETIRE is freed then: the compositor that
//! would have retired it is gone.

use nonos_gfx_wire::body::caps::HW_CURSOR;
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::{Errno, Surface};

use super::display::Display;
use super::platform::Platform;
use super::primary::{next_serial, Primary};
use super::server::Server;

impl<D: Display, P: Platform> Server<D, P> {
    pub fn surface(&mut self, sender: u32, output: u16) -> Result<Surface, Errno> {
        if self.owner != Some(sender) {
            self.owner = Some(sender);
            self.drop_retiring();
        }
        let p = match self.current {
            Some(p) => p,
            None => {
                let st = self.display.state(output)?;
                let p = self.make(st.width, st.height)?;
                self.current = Some(p);
                p
            }
        };
        self.platform.share(p.handle, sender)?;
        Ok(p.surface())
    }

    /// Allocate and register a `w` x `h` primary with a new serial.
    pub fn make(&mut self, w: u32, h: u32) -> Result<Primary, Errno> {
        let cursor = self.display.caps().has(HW_CURSOR);
        let (len, stride, cursor_offset) = Primary::len(w, h, cursor).ok_or(EINVAL)?;
        if w == 0 || h == 0 {
            return Err(EINVAL);
        }
        let region = self.platform.alloc(len)?;
        let handle = match self.platform.register(region, w, h, stride) {
            Ok(h) => h,
            Err(e) => {
                self.platform.release(None, region);
                return Err(e);
            }
        };
        let serial = next_serial(self.platform.pid(), &mut self.counter);
        Ok(Primary { region, handle, serial, width: w, height: h, stride, cursor_offset })
    }
}
