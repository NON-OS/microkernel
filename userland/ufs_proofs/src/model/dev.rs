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

//! The device's answers to NOP OUT and to the fDeviceInit flag queries.

use super::rsp::upiu;
use super::sim::Model;
use crate::ufs::facts::upiu::*;

/// QUERY_RESULT_INVALID_IDN and _INVALID_OPCODE.
const INVALID_IDN: u8 = 0xFD;
const INVALID_OPCODE: u8 = 0xFE;

impl Model {
    pub(super) fn nop(&self) -> Option<Vec<u8>> {
        let cfg = self.cfg.borrow();
        if cfg.nop_hang {
            return None;
        }
        let mut d = self.dev.borrow_mut();
        d.nops += 1;
        let tt = if d.nops <= cfg.nop_wrong { RESPONSE } else { NOP_IN };
        Some(upiu(tt, 0, 0, 0))
    }

    pub(super) fn query(&self, q: &[u8; 32]) -> Option<Vec<u8>> {
        let (func, op, idn) = (q[5], q[12], q[13]);
        let mut d = self.dev.borrow_mut();
        let mut r = upiu(QUERY_RSP, func, 0, 0);
        r[12] = op;
        r[13] = idn;
        d.queries += 1;
        if d.queries <= d.refuse_queries {
            r[6] = INVALID_OPCODE;
            return Some(r);
        }
        if idn != FLAG_DEVICE_INIT {
            r[6] = INVALID_IDN;
            return Some(r);
        }
        match (func, op) {
            (QUERY_WRITE, OP_SET_FLAG) => {
                d.init_flag = true;
                d.init_left = self.cfg.borrow().init_reads;
            }
            (QUERY_READ, OP_READ_FLAG) if d.init_flag && d.init_left > 0 => d.init_left -= 1,
            (QUERY_READ, OP_READ_FLAG) => d.init_flag = false,
            _ => r[6] = INVALID_OPCODE,
        }
        r[23] = d.init_flag as u8;
        Some(r)
    }
}
