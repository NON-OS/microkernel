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

//! Each PDU the peripheral receives: writes kept, requests answered,
//! everything else left alone.

use super::server::Peripheral;
use nonos_bt_wire::att::{error::*, opcode::*};

impl Peripheral {
    pub fn on(&mut self, pdu: &[u8]) -> Option<Vec<u8>> {
        let (&op, p) = pdu.split_first()?;
        let u16_at = |i: usize| u16::from_le_bytes([p[i], p[i + 1]]);
        if op == WRITE_CMD {
            self.writes.push((u16_at(0), p[2..].to_vec(), true));
            return None;
        }
        // A server answers requests only (Vol 3, Part F, 3.3).
        if !is_request(op) || self.silent || self.mute == Some(op) {
            return None;
        }
        self.requests += 1;
        let read = self.refuse_read.filter(|r| op == READ_REQ && r.0 == u16_at(0));
        let refuse = self.refuse.or(self.refuse_op.filter(|r| r.0 == op).map(|r| r.1));
        let refuse = refuse.or(read.map(|r| r.1));
        let result = match (refuse, op) {
            (Some(code), _) => Err((0, code)),
            (_, EXCHANGE_MTU_REQ) => Ok(self.exchange(p)),
            (_, READ_REQ) => self.read(u16_at(0), None),
            (_, READ_BLOB_REQ) => self.read(u16_at(0), Some(u16_at(2))),
            (_, WRITE_REQ) => {
                self.writes.push((u16_at(0), p[2..].to_vec(), false));
                Ok(vec![WRITE_RSP])
            }
            (_, FIND_BY_TYPE_VALUE_REQ) => {
                self.find_by_type(u16_at(0), u16_at(2), &p[4..6], &p[6..])
            }
            (_, READ_BY_TYPE_REQ) => self.read_by_type(u16_at(0), u16_at(2), &p[4..]),
            (_, FIND_INFORMATION_REQ) => self.find_information(u16_at(0), u16_at(2)),
            _ => Err((0, REQUEST_NOT_SUPPORTED)),
        };
        Some(result.unwrap_or_else(|(handle, code)| {
            [&[ERROR_RSP, op][..], &handle.to_le_bytes(), &[code]].concat()
        }))
    }
}
