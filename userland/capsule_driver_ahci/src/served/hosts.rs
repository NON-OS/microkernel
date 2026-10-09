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

//! Every controller found that a disk could be served from.

use alloc::vec::Vec;

use crate::discover::Found as AhciFound;
use crate::emmc::{self, Found as EmmcFound};

/// Every controller this capsule could serve a disk from.
pub struct Hosts {
    pub ahci: Vec<AhciFound>,
    pub emmc: Vec<EmmcFound>,
}

impl Hosts {
    pub fn find() -> Self {
        Self { ahci: crate::discover::find_ahci(), emmc: emmc::discover() }
    }

    pub fn any(&self) -> bool {
        !self.ahci.is_empty() || !self.emmc.is_empty()
    }
}
