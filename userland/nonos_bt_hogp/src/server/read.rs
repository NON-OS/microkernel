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
//! The attribute database this host serves to its peripheral. GAP asks
//! Read By Type (Vol 3, Part F, 3.4.4.1) and Read By Group Type (3.4.4.9).
//! Both answer with a length octet and a list of entries of that length.
//! The list holds only entries as long as the first, and only as many as
//! fit in ATT_MTU (3.4.4.2, 3.4.4.10).

use super::db;
use super::list::{cut, list};
use super::serve::{range, Answer, BAD};
use crate::uuids::{PRIMARY_SERVICE, SECONDARY_SERVICE};
use nonos_bt_wire::att::{error::*, opcode::*, uuid16};
use nonos_bt_wire::cursor::Reader;

/// Handle and value of each attribute of the type, the value cut to
/// ATT_MTU - 4 (3.4.4.2).
pub fn read_by_type(p: &[u8]) -> Answer {
    let mut r = Reader::new(p);
    let (start, end) = range(&mut r)?;
    let kind = uuid(&mut r)?;
    let hits = db::in_range(start, end).filter(|a| Some(a.kind) == kind);
    list(READ_BY_TYPE_RSP, hits.map(|a| (a.handle, None, cut(a.value, 4))), start)
}

/// Handle, group end and value of each service, the value cut to
/// ATT_MTU - 6 (3.4.4.10). Only the two service types group attributes
/// (Vol 3, Part G, 2.5.3); this database has no secondary service.
pub fn read_by_group_type(p: &[u8]) -> Answer {
    let mut r = Reader::new(p);
    let (start, end) = range(&mut r)?;
    match uuid(&mut r)? {
        Some(PRIMARY_SERVICE) => {}
        Some(SECONDARY_SERVICE) => return Err((start, ATTRIBUTE_NOT_FOUND)),
        _ => return Err((start, UNSUPPORTED_GROUP_TYPE)),
    }
    let hits = db::in_range(start, end).filter(|a| a.kind == PRIMARY_SERVICE);
    let entries = hits.map(|a| (a.handle, Some(db::group_end(a)), cut(a.value, 6)));
    list(READ_BY_GROUP_TYPE_RSP, entries, start)
}

/// The attribute type: a 16-bit UUID, or a 128-bit one (3.4.4.1).
fn uuid(r: &mut Reader) -> Result<Option<u16>, (u16, u8)> {
    let field = r.bytes(r.remaining()).map_err(|_| BAD)?;
    match field.len() {
        2 | 16 => Ok(uuid16(field)),
        _ => Err(BAD),
    }
}
