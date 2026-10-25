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

//! The preamble both hellos share: magic, version, suite and flags.

use crate::hello::{MAGIC, SUITE, VERSION};
use crate::{Reader, WireError, Writer};

pub(crate) fn open(body: &[u8]) -> Result<Reader<'_>, WireError> {
    let mut r = Reader::new(body);
    if r.array::<4>()? != &MAGIC {
        return Err(WireError::Magic);
    }
    if r.u16()? != VERSION || r.u16()? != SUITE {
        return Err(WireError::Version);
    }
    r.zero(2).map(|_| r)
}

pub(crate) fn write(out: &mut [u8], random: &[u8], key: &[u8]) -> Result<usize, WireError> {
    let mut w = Writer::new(out);
    w.put(&MAGIC)?;
    w.u16(VERSION)?;
    w.u16(SUITE)?;
    w.u16(0)?;
    w.put(random)?;
    w.put(key)?;
    Ok(w.done())
}
