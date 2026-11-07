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

//! The little-endian fields the nsh/1 formats are made of.

use crate::error::{value, Error};

pub fn u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn bytes8(out: &mut Vec<u8>, b: &[u8], what: &str) -> Result<(), Error> {
    out.push(u8::try_from(b.len()).map_err(|_| value(what))?);
    out.extend_from_slice(b);
    Ok(())
}

/// A capability list as a grant and a table carry it: a count, then each
/// name, in ascending byte order.
pub fn caps(out: &mut Vec<u8>, names: &[String]) -> Result<(), Error> {
    let mut sorted: Vec<&String> = names.iter().collect();
    sorted.sort_unstable();
    sorted.dedup();
    out.push(u8::try_from(sorted.len()).map_err(|_| value("capability list"))?);
    sorted.iter().try_for_each(|n| bytes8(out, n.as_bytes(), "capability name"))
}
