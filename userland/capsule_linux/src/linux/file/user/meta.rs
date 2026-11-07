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

//! A broker name's metadata, as stat and the calls that look first see it.

use nonos_user_files::{Attr, Kind as Node};

use crate::linux::guest::Guest;

use super::super::synth::{S_IFDIR, S_IFREG};
use super::{link, place};

/// The kind and permission bits, the size and the modification time in
/// milliseconds of `full`, when it is in the person's folders and this
/// guest is served them.
pub fn stat(guest: &Guest, full: &[u8]) -> Option<Result<(u32, u64, u64), i64>> {
    let path = place::served(guest, full)?;
    Some(link::with(|c| c.stat(path)).map(|a| shape(&a)).map_err(|e| e as i64))
}

/// Its size and whether it is a directory, or None inside when there is no
/// such name; None outside when it is not the broker's.
pub fn look(guest: &Guest, full: &[u8]) -> Option<Option<(u64, bool)>> {
    let path = place::served(guest, full)?;
    Some(link::with(|c| c.stat(path)).ok().map(|a| (a.size, a.kind == Node::Dir)))
}

fn shape(a: &Attr) -> (u32, u64, u64) {
    let kind = if a.kind == Node::Dir { S_IFDIR } else { S_IFREG };
    (kind | (a.mode & 0o7777), a.size, a.mtime / 1_000_000)
}
