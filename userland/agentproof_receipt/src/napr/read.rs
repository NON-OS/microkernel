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
//! A receipt read, each field held to its length and nothing left over.

use super::cursor::Cursor;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    /// Not `NAPR`, or a version other than 0.
    Head,
    /// A field's length is not its own, or runs past the end.
    Field(&'static str),
    /// Bytes after the doc.
    Trailing,
}

pub fn parse<'a>(b: &'a [u8]) -> Result<Receipt<'a>, Fault> {
    if b.get(..4) != Some(MAGIC) || b.get(4..HEAD) != Some(&VERSION.to_be_bytes()) {
        return Err(Fault::Head);
    }
    let mut c = Cursor { b, at: HEAD };
    let record = c.field("record", RECORD, RECORD)?;
    let box_word = c.field("box", 1, BOX_MAX)?;
    let policy = c.field("app policy", POLICY, POLICY)?;
    let launcher = c.field("launcher", LAUNCHER, LAUNCHER)?;
    let bound = &b[..c.at];
    let doc = c.field("doc", 1, DOC_MAX)?;
    if c.at != b.len() {
        return Err(Fault::Trailing);
    }
    Ok(Receipt {
        record: fixed(record)?,
        box_word,
        policy: fixed(policy)?,
        launcher: fixed(launcher)?,
        doc,
        bound,
    })
}

fn fixed<const N: usize>(v: &[u8]) -> Result<&[u8; N], Fault> {
    v.try_into().map_err(|_| Fault::Field("length"))
}
