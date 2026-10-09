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

use super::region::DmaRegion;
use crate::identity::Names;

pub struct Port {
    pub(crate) clb: DmaRegion,
    pub(crate) ctba: DmaRegion,
    pub(crate) _fb: DmaRegion,
    pub(crate) data: DmaRegion,
    pub(crate) base: u32,
    pub(crate) capacity_sectors: u64,
    /// CAP.SCLO of the port's HBA: recovery may use Command List Override.
    pub(crate) sclo: bool,
    /// Model and serial from the disk's IDENTIFY block.
    pub(crate) names: Names,
    /// PxTFD as the last failed command left it, read before recovery
    /// clears it: the disk's own status and error registers.
    pub(crate) last_tfd: u32,
}
