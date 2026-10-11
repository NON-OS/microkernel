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

//! The one screen this driver serves, as the gfx wire names it: resource 1
//! on scanout 0, held by the first process that asks for it, as
//! driver.virtio_gpu0 holds its primary surface.

use crate::dispi::Mode;
use crate::protocol::{E_BUSY, E_INVAL};

pub const RESOURCE_ID: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub struct Screen {
    pub mode: Mode,
    owner: u32,
}

impl Screen {
    pub const fn new(mode: Mode) -> Self {
        Self { mode, owner: 0 }
    }

    /// GET_PRIMARY_SURFACE: the first caller holds the screen from then on.
    pub fn take(&mut self, pid: u32) -> Result<(), i32> {
        if self.owner == 0 {
            self.owner = pid;
        }
        if self.owner == pid {
            Ok(())
        } else {
            Err(E_BUSY)
        }
    }

    /// A present names this screen's resource and comes from its holder.
    pub fn held(&self, pid: u32, resource: u32) -> Result<(), i32> {
        if resource != RESOURCE_ID {
            return Err(E_INVAL);
        }
        if self.owner != pid {
            Err(E_BUSY)
        } else {
            Ok(())
        }
    }
}
