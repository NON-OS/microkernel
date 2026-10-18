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

//! Flipping the primary plane to another surface of the same layout: one
//! write of PLANE_SURF, latched at the next vertical blank's start, then
//! a wait until PLANE_SURFLIVE shows the new address, so the old surface
//! is never written while it is still being scanned out.

use super::access::{Expiry, Mmio};
use super::error::Error;
use super::pipe::Pipe;
use super::plane::SURF_ALIGN;
use super::plane_regs as r;

pub fn flip(m: &mut dyn Mmio, p: Pipe, surf: u32, until: &dyn Expiry) -> Result<(), Error> {
    if !surf.is_multiple_of(SURF_ALIGN) {
        return Err(Error::Argument);
    }
    m.write(r::plane_surf(p, 1), surf);
    wait_live(m, p, surf, until)
}

/// Until the plane scans out `surf`.
pub fn wait_live(m: &mut dyn Mmio, p: Pipe, surf: u32, until: &dyn Expiry) -> Result<(), Error> {
    loop {
        if m.read(r::plane_surflive(p, 1)) & !0xFFF == surf {
            return Ok(());
        }
        if until.expired() {
            return Err(Error::Timeout);
        }
    }
}
