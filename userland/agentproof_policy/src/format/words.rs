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

//! The words of a `base`, a `write` and a `connect` line.

use super::line::Seen;
use super::Base;
use crate::boxed::dests::list;
use crate::boxed::folders::folder;

/// The most `write` lines a file may have.
pub const MAX_WRITES: usize = 16;
/// The most `connect` lines a file may have: the box's own limit.
pub const MAX_CONNECTS: usize = 64;

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

pub fn connect(seen: &mut Seen, arg: &[u8]) -> Result<(), &'static str> {
    // One destination is one item of the box word: a comma would make it two.
    if arg.contains(&b',') {
        return Err("a connect line names one destination, with no comma");
    }
    let canonical = list(&[arg])?;
    if seen.connects.iter().any(|(_, c)| list(&[c]).ok().as_ref() == Some(&canonical)) {
        return Err("a destination is named once");
    }
    if seen.connects.len() == MAX_CONNECTS {
        return Err("a file has at most 64 connect lines");
    }
    seen.connects.push((seen.line, arg.to_vec()));
    Ok(())
}
