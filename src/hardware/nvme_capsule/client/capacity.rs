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

use super::super::capability::gate_call;
use super::super::error::DriverNvmeError;
use super::layout::layout;
use super::lba_map::capacity_sectors;

/// The namespace's size in 512-byte sectors, whatever its LBA size.
pub fn capacity() -> Result<u64, DriverNvmeError> {
    let _caller = gate_call()?;
    let layout = layout()?;
    capacity_sectors(layout.capacity_lbas, layout.lba_size).ok_or(DriverNvmeError::Unsupported)
}
