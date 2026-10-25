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

//! A faulting device blocked on AMD-Vi: its device table entry set to
//! deny every access, interrupts kept, and the unit's copy dropped. Only a
//! device in a live capsule domain; one firmware's reserved memory keeps is
//! left as it is.

use super::super::devtab::{read_dte, update_dte};
use super::super::dte::{blocked, domain_of, keep_interrupts};
use super::super::error::AmdViError;
use super::super::flush::flush_device;
use super::identity::note;
use super::lifetime::{root_of, ROOTS};

/// True when a device in a capsule domain was blocked. Its release then
/// finds it denied already, as detach would leave it.
pub fn block(id: u16) -> Result<bool, AmdViError> {
    let roots = ROOTS.lock();
    let current = read_dte(id)?;
    if root_of(&roots, domain_of(current)).is_err() {
        return Ok(false);
    }
    let (before, after) = update_dte(id, |now| keep_interrupts(blocked(), now))?;
    note(before, after);
    flush_device(id)?;
    Ok(true)
}
