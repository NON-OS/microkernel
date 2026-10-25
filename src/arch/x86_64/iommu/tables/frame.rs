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

use crate::memory::frame_alloc::manager::allocate_frame;

use super::super::types::VtdError;
use super::view::Frame;

/// A fresh table must read as all zeroes before any device can reach it: a
/// second-level entry with no permission bits denies rather than faults, so
/// zero is the only safe initial state.
pub fn allocate_table() -> Result<u64, VtdError> {
    let phys = allocate_frame().ok_or(VtdError::PageTableExhausted)?;
    Frame::at(phys.as_u64())?.zero();
    super::publish::publish(phys.as_u64());
    Ok(phys.as_u64())
}
