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

//! A screen for the pool's server: one output whose mode the compositor may
//! set within VGA to XGA, a width a multiple of 8, and whose flushes are
//! taken whole (`screen_display`). What it shows is not the point here;
//! the primaries are.

use nonos_gfx_wire::errno::ENXIO;
use nonos_gfx_wire::Errno;

pub struct Screen {
    pub size: (u32, u32),
    pub flushes: usize,
}

impl Screen {
    pub fn new() -> Self {
        Screen { size: (640, 480), flushes: 0 }
    }

    pub(super) fn output(output: u16) -> Result<(), Errno> {
        if output == 0 {
            Ok(())
        } else {
            Err(ENXIO)
        }
    }
}
