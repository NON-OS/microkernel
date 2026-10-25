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

//! An adapter that does not take a mode wider than its video memory holds a
//! line of: it clamps the width it is given, as QEMU's does past its
//! maximum, so the read-back finds another mode than the one written. The
//! clamp answers each write as it is made (`on_w16`); as a thread of its
//! own it raced the read-back, and lost whenever the driver kept its CPU.

use std::sync::Arc;

use nonos_libc::on_w16;

use super::adapter::Adapter;
use crate::constants::DISPI_INDEX_XRES;
use crate::dispi::dispi_off;

/// The widest mode the clamping adapter shows.
pub const MAX_XRES: u16 = 800;

/// From now on `a` reads back any XRES written above `MAX_XRES` as it.
pub fn clamping(a: &Adapter) {
    let (regs, xres) = (Arc::clone(&a.regs), dispi_off(DISPI_INDEX_XRES) as usize);
    let at = a.regs.base() + xres as u64;
    on_w16(move |addr, value| {
        if addr == at && value > MAX_XRES {
            regs.present16(xres, MAX_XRES);
        }
    });
}
