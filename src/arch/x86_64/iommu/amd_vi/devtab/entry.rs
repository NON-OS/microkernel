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

use core::sync::atomic::{compiler_fence, Ordering};

use spin::Mutex;

use super::super::dte::Dte;
use super::super::error::AmdViError;
use super::table::{device_table, PER_PAGE};
use crate::arch::x86_64::iommu::tables::view::Frame;

/// Replace the entry for `device_id`, the first quadword (V, TV, mode and
/// root) last, so a unit never reads a new root beside an old domain. The
/// caller flushes the unit's cached copy afterwards.
fn write_dte(device_id: u16, entry: Dte) -> Result<(), AmdViError> {
    let (words, at) = slot_of(device_id)?;
    words.write(at + 3, entry[3])?;
    words.write(at + 2, entry[2])?;
    words.write(at + 1, entry[1])?;
    compiler_fence(Ordering::SeqCst);
    words.write(at, entry[0])?;
    Ok(())
}

/* Held across each read and rewrite, so a domain move and an interrupt
 * table's arrival never write over each other's half of an entry. */
static UPDATE: Mutex<()> = Mutex::new(());

/// Rewrite `device_id`'s entry as `change` makes it from the current one;
/// returns the entry before and after.
pub(in super::super) fn update_dte(
    device_id: u16,
    change: impl FnOnce(Dte) -> Dte,
) -> Result<(Dte, Dte), AmdViError> {
    let _held = UPDATE.lock();
    let before = read_dte(device_id)?;
    let after = change(before);
    write_dte(device_id, after)?;
    Ok((before, after))
}

pub(in super::super) fn read_dte(device_id: u16) -> Result<Dte, AmdViError> {
    let (words, at) = slot_of(device_id)?;
    Ok([words.read(at)?, words.read(at + 1)?, words.read(at + 2)?, words.read(at + 3)?])
}

/// The page holding `device_id`'s entry, and the entry's first quadword in it.
fn slot_of(device_id: u16) -> Result<(Frame, usize), AmdViError> {
    let page = device_table()? + (device_id as usize / PER_PAGE) as u64 * 4096;
    let words = Frame::at(page).map_err(|_| AmdViError::TableUnreachable)?;
    Ok((words, (device_id as usize % PER_PAGE) * 4))
}
