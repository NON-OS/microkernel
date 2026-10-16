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

//! A bind's last step (L69): its grants entered only while its claim stands.
//! An exit drops the claim and then drains the records, so a grant entered
//! after the drain would keep a vector routed for a capsule that is gone; one
//! refused has what the bind programmed undone instead (`undo_refused`).

use super::super::grant::IrqGrant;
use super::super::records;
use super::super::release::undo_refused;
use super::super::types::IrqBindError;

pub(super) fn enter(grants: &[IrqGrant]) -> Result<(), IrqBindError> {
    if records::insert_live(grants) {
        return Ok(());
    }
    undo_refused(grants);
    Err(IrqBindError::NotClaimed)
}
