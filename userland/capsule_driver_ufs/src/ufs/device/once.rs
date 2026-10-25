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

//! One NOP OUT or flag query and its answer, which must echo the slot, the
//! unit and, for a query, the opcode and IDN it was asked (JESD220 UPIU
//! headers, as recalled, unverified).

use super::ask::Ask;
use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::facts::upiu::{NOP_IN, QUERY_RSP, TARGET_SUCCESS as OK};
use crate::ufs::slot::descriptor::Dir;
use crate::ufs::slot::run::{run, Req};
use crate::ufs::upiu::request::{flag_query, nop_out};
use crate::ufs::upiu::response::Rsp;

pub(super) fn once<H: Hw>(hw: &H, c: &Ctx, a: Ask, ms: u64) -> Result<bool, UfsError> {
    let upiu = match a {
        Ask::Nop => nop_out(),
        Ask::Flag { func, op, idn } => flag_query(func, op, idn),
    };
    let req = Req { cmd_type: c.dev_type(), dir: Dir::None, upiu: &upiu, data_len: 0, ms };
    let rsp = run(hw, c, &req)?;
    let r = Rsp(&rsp);
    let tt = if let Ask::Nop = a { NOP_IN } else { QUERY_RSP };
    // An answer for another slot, or to another query, is not this one's.
    let echoed = match a {
        Ask::Nop => true,
        Ask::Flag { op, idn, .. } => r.echoes(op, idn),
    };
    if !r.answers(tt, 0) || !echoed {
        return Err(UfsError::Upiu(r.tt()));
    }
    // A query's response field is its response code.
    match (a, r.tt(), r.response()) {
        (Ask::Nop, NOP_IN, OK) | (Ask::Flag { .. }, QUERY_RSP, OK) => Ok(r.flag()),
        (Ask::Flag { .. }, QUERY_RSP, code) => Err(UfsError::Query(code)),
        (_, tt, _) => Err(UfsError::Upiu(tt)),
    }
}
