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
//! Find Information (Vol 3, Part F, 3.4.3.1) and Find By Type Value
//! (3.4.3.3): handles and types, or handles and group ends, as many as
//! fit in one response.

use alloc::vec::Vec;

use super::db;
use super::serve::{range, Answer, ATT_MTU, BAD};
use nonos_bt_wire::att::{error::ATTRIBUTE_NOT_FOUND, opcode::*};
use nonos_bt_wire::cursor::Reader;

/// Find Information Response, format 1: every type here is a 16-bit UUID
/// (3.4.3.2).
pub fn find_information(p: &[u8]) -> Answer {
    let mut r = Reader::new(p);
    let (start, end) = range(&mut r)?;
    r.finish().map_err(|_| BAD)?;
    let pairs = db::in_range(start, end).map(|a| (a.handle, a.kind));
    let mut out = alloc::vec![FIND_INFORMATION_RSP, 0x01];
    fill(&mut out, pairs);
    found(out, 2, start)
}

/// Find By Type Value Response: the handle of each attribute of that type
/// and value, and the end of the group it opens (3.4.3.4).
pub fn find_by_type_value(p: &[u8]) -> Answer {
    let mut r = Reader::new(p);
    let (start, end) = range(&mut r)?;
    let kind = r.u16().map_err(|_| BAD)?;
    let value = r.bytes(r.remaining()).map_err(|_| BAD)?;
    let hits = db::in_range(start, end).filter(|a| a.kind == kind && a.value == value);
    let mut out = alloc::vec![FIND_BY_TYPE_VALUE_RSP];
    fill(&mut out, hits.map(|a| (a.handle, db::group_end(a))));
    found(out, 1, start)
}

/// Append four-octet pairs while the response stays within ATT_MTU.
fn fill(out: &mut Vec<u8>, pairs: impl Iterator<Item = (u16, u16)>) {
    for (a, b) in pairs {
        if out.len() + 4 > ATT_MTU {
            break;
        }
        out.extend_from_slice(&a.to_le_bytes());
        out.extend_from_slice(&b.to_le_bytes());
    }
}

/// The response, or Attribute Not Found on the start if nothing was found.
pub fn found(out: Vec<u8>, header: usize, start: u16) -> Answer {
    if out.len() == header {
        return Err((start, ATTRIBUTE_NOT_FOUND));
    }
    Ok(out)
}
