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

//! Where a pull lands in the store when no destination is named.

use alloc::vec::Vec;

use super::Target;

pub fn default_dest(t: &Target) -> Vec<u8> {
    let trimmed = if t.is_dir { &t.path[..t.path.len() - 1] } else { &t.path[..] };
    if trimmed.is_empty() {
        let mut d = t.hostname.clone();
        if t.is_dir {
            d.push(b'/');
        }
        return d;
    }
    match trimmed.iter().rposition(|&c| c == b'/') {
        Some(i) => {
            let mut d = t.path[i + 1..].to_vec();
            if d.is_empty() {
                d.push(b'.');
            }
            d
        }
        None => t.path.clone(),
    }
}
