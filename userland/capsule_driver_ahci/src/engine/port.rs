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
use crate::clock::Deadline;
use crate::identity::Names;

pub struct Port {
    pub(crate) clb: DmaRegion,
    pub(crate) ctba: DmaRegion,
    pub(crate) _fb: DmaRegion,
    pub(crate) data: DmaRegion,
    pub(crate) base: u32,
    pub(crate) capacity_sectors: u64,
    /// Model and serial from the disk's IDENTIFY block.
    pub(crate) names: Names,
    /// PxTFD as the last failed command left it, read before recovery
    /// clears it: the disk's own status and error registers.
    pub(crate) last_tfd: u32,
    /// Recovery could not stop the command engine: nothing more is built,
    /// copied into the data buffer, or issued on this port.
    pub(crate) wedged: bool,
    /// Each command's budget: BRING_UP_COMMAND_MS while the walk tries the
    /// disk, COMMAND_MS once it is served.
    pub(crate) command_ms: u64,
    /// When ATA's window for the drive to come out of the last COMRESET a
    /// recovery sent closes (RESET_WINDOW_MS); `None` before any. Until
    /// then a drive that shows BSY or DRQ is still in that reset, and is
    /// waited out (`prepare`).
    pub(crate) reset_window: Option<Deadline>,
}
