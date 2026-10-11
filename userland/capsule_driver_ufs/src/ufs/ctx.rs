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

use crate::ufs::hci::caps::Caps;
use crate::ufs::utp::layout::Lists;
use crate::ufs::utp::utrd::{TYPE_DEV_MANAGE, TYPE_SCSI, TYPE_UFS_STORAGE};

#[derive(Clone, Copy, Debug)]
pub struct Ctx {
    pub lists: Lists,
    pub caps: Caps,
}

impl Ctx {
    /// The UTRD command type of a SCSI command (ufshcd_prepare_req_desc_hdr).
    pub fn scsi_type(&self) -> u32 {
        if self.caps.v1x {
            TYPE_SCSI
        } else {
            TYPE_UFS_STORAGE
        }
    }

    /// The UTRD command type of a NOP OUT or a query (ufshcd_compose_dev_cmd).
    pub fn dev_type(&self) -> u32 {
        if self.caps.v1x {
            TYPE_DEV_MANAGE
        } else {
            TYPE_UFS_STORAGE
        }
    }
}
