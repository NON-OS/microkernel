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

//! One entry of the shared device table, read and written in place.

use core::sync::atomic::{compiler_fence, Ordering};

use super::devtab::{device_table, PER_PAGE};
use super::dte::Dte;
use super::error::AmdViError;
use crate::arch::x86_64::iommu::tables::frame::entries_mut;

/// Replace the entry for `device_id`, the first quadword (V, TV, mode and
/// root) last, so a unit never reads a new root beside an old domain. The
/// caller flushes the unit's cached copy afterwards.
pub(super) fn write_dte(device_id: u16, entry: Dte) -> Result<(), AmdViError> {
    let (words, at) = slot_of(device_id)?;
    words[at + 3] = entry[3];
    words[at + 2] = entry[2];
    words[at + 1] = entry[1];
    compiler_fence(Ordering::SeqCst);
    words[at] = entry[0];
    // The entry is 32 bytes, 32-aligned: one line (K12).
    super::visible::flush(&words[at]);
    Ok(())
}

pub(super) fn read_dte(device_id: u16) -> Result<Dte, AmdViError> {
    let (words, at) = slot_of(device_id)?;
    Ok([words[at], words[at + 1], words[at + 2], words[at + 3]])
}

/// The page holding `device_id`'s entry, and the entry's first quadword in it.
fn slot_of(device_id: u16) -> Result<(&'static mut [u64], usize), AmdViError> {
    let page = device_table()? + (device_id as usize / PER_PAGE) as u64 * 4096;
    let words = entries_mut(page).map_err(|_| AmdViError::TableUnreachable)?;
    Ok((words, (device_id as usize % PER_PAGE) * 4))
}
