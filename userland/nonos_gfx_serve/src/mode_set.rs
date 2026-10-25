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
//! primary waiting for RETIRE per output; a refused MODE_SET keeps the
//! old mode and frees whatever it made.

use nonos_gfx_wire::errno::EBUSY;
use nonos_gfx_wire::{Errno, Surface};

use super::display::Display;
use super::mode_check::check;
use super::platform::Platform;
use super::primary::Primary;
use super::server::Server;

impl<D: Display, P: Platform> Server<D, P> {
    pub fn mode_set(&mut self, sender: u32, output: u16, w: u32, h: u32) -> Result<Surface, Errno> {
        if self.slot(output)?.retiring.is_some() {
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
        let slot = self.slot(output)?;
        slot.retiring = slot.current.replace(new);
        self.owner = Some(sender);
        Ok(new.surface())
    }

    /// RETIRE: the compositor no longer reads the old primary. A serial
    /// names one primary across every output, so any output's is found.
    pub fn retire(&mut self, serial: u32) -> Result<(), Errno> {
        use nonos_gfx_wire::errno::{EINVAL, ESTALE};
        let named = |p: Option<Primary>| p.is_some_and(|p| p.serial == serial);
        if self.slots.iter().any(|s| named(s.current)) {
            return Err(EINVAL);
        }
        let o = self.slots.iter().position(|s| named(s.retiring)).ok_or(ESTALE)?;
        self.drop_retiring(o);
        Ok(())
    }
}
