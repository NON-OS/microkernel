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

use crate::object_id::object_has_id;

/// Return the top-level response object whose `id` matches, so the ordinary
/// `parse_*` functions can extract its `result`. String contents are skipped so
/// a brace or quote inside a value never confuses the object boundaries.
pub fn object_for_id(resp: &[u8], id: u64) -> Option<&[u8]> {
    let mut in_string = false;
    let mut escaped = false;
    let mut depth = 0u32;
    let mut start = 0usize;
    for (i, &b) in resp.iter().enumerate() {
        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'{' => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let obj = &resp[start..=i];
                    if object_has_id(obj, id) {
                        return Some(obj);
                    }
                }
            }
            _ => {}
        }
    }
    None
}
