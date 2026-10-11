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

//! ufshcd_query_flag_retry: one flag read or set, tried QUERY_REQ_RETRIES
//! (3) times, each bounded by QUERY_REQ_TIMEOUT.

use super::send::send;
use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::time::QUERY_MS;
use crate::ufs::upiu::codes::QUERY_RSP;
use crate::ufs::upiu::request::flag_query;
use crate::ufs::upiu::response::Rsp;

const TRIES: usize = 3;

/// The flag's value as the device answered (for a set, what it holds now).
pub fn flag<H: Hw>(hw: &H, c: &Ctx, func: u8, op: u8, idn: u8) -> Result<bool, UfsError> {
    let mut last = UfsError::Timeout;
    for _ in 0..TRIES {
        match once(hw, c, func, op, idn) {
            Ok(v) => return Ok(v),
            Err(e) => last = e,
        }
    }
    Err(last)
}

fn once<H: Hw>(hw: &H, c: &Ctx, func: u8, op: u8, idn: u8) -> Result<bool, UfsError> {
    let rsp = send(hw, c, &flag_query(func, op, idn), QUERY_MS)?;
    let r = Rsp(&rsp);
    if r.tt() != QUERY_RSP {
        return Err(UfsError::Upiu(r.tt()));
    }
    match r.response() {
        0 => Ok(r.flag()),
        code => Err(UfsError::Query(code)),
    }
}
