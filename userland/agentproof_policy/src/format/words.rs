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

//! The words of a `base` line and a `write` line.

use super::line::Seen;
use super::Base;
use crate::boxed::folders::folder;

/// The most `write` lines a file may have.
pub const MAX_WRITES: usize = 16;

pub fn base(word: &[u8]) -> Result<Base, &'static str> {
    match word {
        b"strict" => Ok(Base::Strict),
        b"offline" => Ok(Base::Offline),
        b"folders" => Ok(Base::Folders),
        _ => Err("base is strict, offline or folders"),
    }
}

pub fn write(seen: &mut Seen, arg: &[u8]) -> Result<(), &'static str> {
    let f = folder(arg)?;
    if seen.writes.iter().any(|(_, w)| *w == f) {
        return Err("a folder is named once");
    }
    if seen.writes.len() == MAX_WRITES {
        return Err("a file has at most 16 write lines");
    }
    seen.writes.push((seen.line, f));
    Ok(())
}
