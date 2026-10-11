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

//! ufshcd_verify_dev_init: NOP OUT until the device answers NOP IN, at most
//! five times. A timeout ends it at once, as in Linux: a device that does
//! not answer one NOP OUT in 50 ms is not about to answer the next.

use super::send::send;
use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::time::NOP_MS;
use crate::ufs::upiu::codes::{NOP_IN, TARGET_SUCCESS};
use crate::ufs::upiu::request::nop_out;
use crate::ufs::upiu::response::Rsp;

/// NOP_OUT_RETRIES.
const TRIES: usize = 5;

pub fn verify<H: Hw>(hw: &H, c: &Ctx) -> Result<(), UfsError> {
    let mut last = UfsError::Timeout;
    for _ in 0..TRIES {
        match send(hw, c, &nop_out(), NOP_MS) {
            Ok(rsp) if Rsp(&rsp).tt() == NOP_IN && Rsp(&rsp).response() == TARGET_SUCCESS => {
                return Ok(());
            }
            Ok(rsp) => last = UfsError::Upiu(Rsp(&rsp).tt()),
            Err(UfsError::Timeout) => return Err(UfsError::Timeout),
            Err(e) => last = e,
        }
    }
    Err(last)
}
