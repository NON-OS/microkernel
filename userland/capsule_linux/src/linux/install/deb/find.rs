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

//! Which Debian record an install takes for a name.

use super::packages::Record;

/// The package the person asked for by its own name only, so its pin is
/// never spent on, or skipped for, a provider; a dependency by its own
/// name first, then by a name it provides.
pub fn find<'a>(index: &'a [Record], want: &str, listed: bool) -> Option<&'a Record> {
    let named = index.iter().find(|r| r.name == want);
    match listed {
        true => named,
        false => named.or_else(|| index.iter().find(|r| r.provides.iter().any(|p| p == want))),
    }
}
