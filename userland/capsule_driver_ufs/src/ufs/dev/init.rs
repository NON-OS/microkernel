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

//! ufshcd_complete_dev_init: fDeviceInit set, then read until the device
//! clears it, which it does once its own initialisation is through. Reads
//! are a millisecond apart, as Linux's usleep_range(500, 1000).

use super::query::flag;
use crate::ufs::ctx::Ctx;
use crate::ufs::env::{pause, Deadline, Hw};
use crate::ufs::error::UfsError;
use crate::ufs::time::DEVICE_INIT_MS;
use crate::ufs::upiu::codes::*;

pub fn complete<H: Hw>(hw: &H, c: &Ctx) -> Result<(), UfsError> {
    flag(hw, c, QUERY_WRITE, OP_SET_FLAG, FLAG_DEVICE_INIT)?;
    let d = Deadline::after(hw, DEVICE_INIT_MS);
    loop {
        if !flag(hw, c, QUERY_READ, OP_READ_FLAG, FLAG_DEVICE_INIT)? {
            return Ok(());
        }
        if d.passed(hw) {
            return Err(UfsError::Timeout);
        }
        pause(hw, 1);
    }
}
