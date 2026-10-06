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

extern crate alloc;

use alloc::string::String;

use super::tags::TagMap;

/// What `tag_commit` does to the map, and the words for it.
pub fn toggle_tag(tags: &mut TagMap, paths: &[String], name: &str) -> &'static [u8] {
    let lower = name.to_ascii_lowercase();
    let all_have = paths.iter().all(|p| tags.tags_for(p).contains(&lower.as_str()));
    if all_have {
        for p in paths {
            tags.remove(p, name);
        }
        return b"untagged";
    }
    let refused = paths.iter().filter(|p| !tags.add(p, name)).count();
    match refused {
        0 => b"tagged",
        n if n == paths.len() => b"not tagged: up to 24 letters, digits or -, and 8 tags a file",
        _ => b"tagged; some had no room for another tag",
    }
}
