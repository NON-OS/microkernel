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

//! One lighting of a taken-over output at a timing on the surfaces at
//! `bases`: the port relit, the plane's buffer and watermarks for the new
//! line, then the plane rebased onto the surfaces inside the output's GGTT
//! window, live before it returns.

use nonos_gfx_wire::errno::{EINVAL, ENOMEM, ETIMEDOUT};
use nonos_gfx_wire::Errno;

use super::intel::FLIP_US;
use super::modeset::Modeset;
use crate::engine::access::{Clock, Mmio, Until};
use crate::engine::error::Error;
use crate::engine::relight::{buffer, relight};
use crate::engine::scanout::Scanout;
use nonos_edid::Detailed;

pub fn light(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    scan: &mut Scanout,
    (ms, timing): (&Modeset, &Detailed),
    size: (u32, u32),
    bases: [u64; 2],
) -> Result<(), Error> {
    relight(m, clock, &ms.lit, timing, ms.tmds_khz)?;
    buffer(m, ms.lit.p, size.0, timing, &ms.latency_us)?;
    let until = Until::new(clock, FLIP_US);
    scan.rebase(m, size, ms.window, bases, &until)
}

/// What a failed lighting answers on the wire.
pub fn errno(e: Error) -> Errno {
    match e {
        Error::Timeout => ETIMEDOUT,
        Error::GgttFull => ENOMEM,
        _ => EINVAL,
    }
}
