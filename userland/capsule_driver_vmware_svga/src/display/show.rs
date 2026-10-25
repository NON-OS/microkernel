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

//! A mode set, held to the mapping and put back when the adapter did not
//! take it.

use super::svga::{Shown, Svga};
use crate::init::set_mode;
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::Errno;

impl Svga {
    pub fn program(&mut self, w: u32, h: u32) -> Result<(), Errno> {
        if !self.fits(w, h) {
            return Err(EINVAL);
        }
        let taken = set_mode(&self.regs, (w, h)).ok();
        let shown = taken.map(|l| Shown { width: w, height: h, pitch: l.pitch, offset: l.offset });
        match shown.filter(|&m| self.laid_inside(m)) {
            Some(m) => {
                self.mode = m;
                Ok(())
            }
            None => {
                let old = self.mode;
                let _ = set_mode(&self.regs, (old.width, old.height));
                Err(EINVAL)
            }
        }
    }
}
