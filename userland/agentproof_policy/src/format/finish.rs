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

//! What a whole file must have said once its last line is read.

use super::line::Seen;
use super::{Base, Error, Policy};

pub fn finish(seen: Seen) -> Result<Policy, Error> {
    let whole = |why| Err(Error { line: 0, why });
    if !seen.header {
        return whole("the first line is `nonos-box 1`");
    }
    let Some(base) = seen.base else { return whole("base is missing") };
    if !seen.network {
        return whole("network is missing; write `network none`");
    }
    match (base, seen.writes.first()) {
        (Base::Folders, None) => whole("base folders needs at least one write line"),
        (Base::Strict | Base::Offline, Some((line, _))) => Err(Error {
            line: *line,
            why: "write lines go with base folders; strict and offline name their own",
        }),
        _ => Ok(Policy { base, writes: seen.writes.into_iter().map(|(_, w)| w).collect() }),
    }
}
