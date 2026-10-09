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

//! The `C:` field of an index record: the SHA-1 of a package's control
//! member, written `Q1` and then base64.

use super::base64::decode;

pub fn parse(field: &str) -> Option<[u8; 20]> {
    let raw = decode(field.strip_prefix("Q1")?.as_bytes())?;
    raw.try_into().ok()
}
