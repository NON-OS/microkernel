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

use alloc::string::String;

use super::{find_close, split_top_comma};

/* light-dark() calls rewritten in one value, and nested in each other;
 * the rest stay as written. */
const MAX_CALLS: u32 = 8;

/* Every light-dark(a, b) rewritten to its light argument, which may hold
 * a nested light-dark() of its own. */
pub(super) fn strip_light_dark(value: &str) -> String {
    strip(value, 0)
}

fn strip(value: &str, depth: u32) -> String {
    let (mut out, mut cursor, mut calls) = (String::new(), 0, depth);
    while let Some(rel) = value[cursor..].find("light-dark(") {
        calls += 1;
        if calls > MAX_CALLS {
            break;
        }
        let p = cursor + rel;
        out.push_str(&value[cursor..p]);
        let Some(close) = find_close(value, p + 11) else {
            out.push_str(&value[p..]);
            return out;
        };
        let (light, _) = split_top_comma(&value[p + 11..close]);
        let light = light.trim();
        match light.contains("light-dark(") {
            true => out.push_str(&strip(light, calls)),
            false => out.push_str(light),
        }
        cursor = close + 1;
    }
    out.push_str(&value[cursor..]);
    out
}
