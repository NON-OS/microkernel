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

//! What every request needs once the lists run: the regions, and the
//! command types this host's UFSHCI version takes.

use core::cell::Cell;

use crate::ufs::env::Hw;
use crate::ufs::facts::hci::UTRLDBR;
use crate::ufs::facts::utp::{TYPE_DEV_MANAGE, TYPE_SCSI, TYPE_UFS_STORAGE};
use crate::ufs::host::caps::Caps;
use crate::ufs::slot::layout::{Lists, SLOT};

#[derive(Clone, Debug)]
pub struct Ctx {
    pub lists: Lists,
    pub caps: Caps,
    /// The slot is still the host's: a request it never finished and a
    /// clear it never took. Latched, so no later request writes the command
    /// descriptor or the data region an old READ may still land in.
    pub wedged: Cell<bool>,
}

impl Ctx {
    pub fn new(lists: Lists, caps: Caps) -> Ctx {
        Ctx { lists, caps, wedged: Cell::new(false) }
    }

    /// Whether the slot may not be used: latched, or its doorbell bit still
    /// set when no request of ours is in flight, which latches it.
    pub fn slot_held<H: Hw>(&self, hw: &H) -> bool {
        if !self.wedged.get() && hw.r32(UTRLDBR) & 1 << SLOT != 0 {
            self.wedged.set(true);
        }
        self.wedged.get()
    }

    /// The UTRD command type of a SCSI command, by UFSHCI version.
    pub fn scsi_type(&self) -> u32 {
        if self.caps.v1x {
            TYPE_SCSI
        } else {
            TYPE_UFS_STORAGE
        }
    }

    /// The UTRD command type of a NOP OUT or a query, by UFSHCI version.
    pub fn dev_type(&self) -> u32 {
        if self.caps.v1x {
            TYPE_DEV_MANAGE
        } else {
            TYPE_UFS_STORAGE
        }
    }
}
