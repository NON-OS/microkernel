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

//! Finding the extension that names the signing key.

use super::super::constants::EXT_SIGNED_WITH_KEY;

pub(super) fn signing_key(mut rest: &[u8], count: u8) -> Option<[u8; 32]> {
    for _ in 0..count {
        if rest.len() < 4 {
            return None;
        }
        let len = u16::from_be_bytes([rest[0], rest[1]]) as usize;
        let data = rest.get(4..4 + len)?;
        if rest[2] == EXT_SIGNED_WITH_KEY && len == 32 {
            let mut key = [0u8; 32];
            key.copy_from_slice(data);
            return Some(key);
        }
        rest = rest.get(4 + len..)?;
    }
    None
}
