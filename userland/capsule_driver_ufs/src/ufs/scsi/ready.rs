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

//! TEST UNIT READY until the unit is: a unit reports UNIT ATTENTION once
//! after a reset and NOT READY while it comes up, and is asked again ten
//! milliseconds later until UNIT_READY_MS has passed.

use super::cdb::test_unit_ready;
use super::codes::{KEY_NOT_READY, KEY_UNIT_ATTENTION};
use super::exec::{exec, Data};
use crate::ufs::ctx::Ctx;
use crate::ufs::env::{pause, Deadline, Hw};
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::{SCSI_MS, UNIT_READY_MS};

pub fn wait<H: Hw>(hw: &H, c: &Ctx, lun: u8) -> Result<(), UfsError> {
    let d = Deadline::after(hw, UNIT_READY_MS);
    loop {
        match exec(hw, c, lun, test_unit_ready(), Data::None, SCSI_MS) {
            Err(UfsError::Sense { key: KEY_NOT_READY | KEY_UNIT_ATTENTION, .. })
                if !d.passed(hw) =>
            {
                pause(hw, 10)
            }
            r => return r,
        }
    }
}
