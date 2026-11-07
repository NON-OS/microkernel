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

//! SCANOUT: only the project (the compositor) may put a client's surface on
//! a plane, only on a driver that keeps the band on a plane of its own,
//! and only against the output's current primary, which the band is read
//! from: a stale serial is ESTALE, as FLUSH's is.

use nonos_gfx_wire::body::caps::{BAND_PLANE, DIRECT_SCANOUT};
use nonos_gfx_wire::errno::{EINVAL, ENOSYS, EPERM, ESTALE};
use nonos_gfx_wire::{Errno, Scanout};

use super::display::Display;
use super::platform::Platform;
use super::server::Server;

impl<D: Display, P: Platform> Server<D, P> {
    pub(super) fn scanout(&mut self, sender: u32, output: u16, s: &Scanout) -> Result<(), Errno> {
        if self.owner != Some(sender) {
            return Err(EPERM);
        }
        let flags = self.display.caps().flags;
        if flags & DIRECT_SCANOUT == 0 || flags & BAND_PLANE == 0 {
            return Err(ENOSYS);
        }
        let cur = self.slot(output)?.current.filter(|c| c.serial == s.serial).ok_or(ESTALE)?;
        if (s.width, s.height) != (cur.width, cur.height) {
            return Err(EINVAL);
        }
        self.with_shadow(cur, |d, shadow| d.scanout(output, s, shadow))
    }
}
