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

//! MODE_SET makes before it breaks (section 12): the new primary is made,
//! shown, shared and answered while the old one stays until RETIRE. One
//! primary waiting for RETIRE at a time; a refused MODE_SET keeps the old
//! mode and frees whatever it made.

use nonos_gfx_wire::errno::EBUSY;
use nonos_gfx_wire::{Errno, Surface};

use super::display::Display;
use super::mode_check::check;
use super::platform::Platform;
use super::server::Server;

impl<D: Display, P: Platform> Server<D, P> {
    pub fn mode_set(&mut self, sender: u32, output: u16, w: u32, h: u32) -> Result<Surface, Errno> {
        if self.retiring.is_some() {
            return Err(EBUSY);
        }
        check(&self.display, output, w, h)?;
        let new = self.make(w, h)?;
        let shown = self.with_shadow(new, |d, s| d.set_mode(output, w, h, s));
        let shared = shown.and_then(|_| self.platform.share(new.handle, sender));
        if let Err(e) = shared {
            self.platform.release(Some(new.handle), new.region);
            return Err(e);
        }
        self.retiring = self.current.replace(new);
        self.owner = Some(sender);
        Ok(new.surface())
    }

    /// RETIRE: the compositor no longer reads the old primary.
    pub fn retire(&mut self, serial: u32) -> Result<(), Errno> {
        use nonos_gfx_wire::errno::{EINVAL, ESTALE};
        match (self.current, self.retiring) {
            (Some(c), _) if c.serial == serial => Err(EINVAL),
            (_, Some(r)) if r.serial == serial => {
                self.drop_retiring();
                Ok(())
            }
            _ => Err(ESTALE),
        }
    }
}
