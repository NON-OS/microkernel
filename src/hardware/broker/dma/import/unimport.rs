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

//! `MkDmaUnimportSurface`: the holder gives one import back. The device has
//! lost the surface, its IOTLB entries invalidated, when this returns.

use super::records::RECORDS;
use super::table_rule::{take_one, UnimportError};

pub fn unimport_surface(pid: u32, import_id: u64) -> Result<(), UnimportError> {
    let gone = take_one(&mut RECORDS.lock(), pid, import_id)?;
    super::drop::take_out(&gone);
    Ok(())
}
