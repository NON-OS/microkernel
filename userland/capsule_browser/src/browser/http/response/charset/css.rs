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

use super::encoding::Encoding;
use super::label::encoding;

/* The encoding a stylesheet declares (CSS Syntax 3.2): its first 1024
bytes start with exactly `@charset "`, a label, then `";`. UTF-16
there means UTF-8. */
pub fn css_charset(b: &[u8]) -> Option<Encoding> {
    let rest = b.strip_prefix(b"@charset \"")?;
    let end = rest.iter().take(1024 - 11).position(|&c| c == b'"' || c == b';')?;
    if rest.get(end..end + 2) != Some(b"\";") {
        return None;
    }
    encoding(&rest[..end]).map(Encoding::ascii_compatible)
}
