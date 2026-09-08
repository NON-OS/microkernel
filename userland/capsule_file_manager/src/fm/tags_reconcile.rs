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

// Drop tag assignments whose path no longer exists. Scoped to `prefix` because
// `live` only lists the directory currently loaded, so a global sweep would
// delete every tag outside the view.
pub fn reconcile(map: &mut TagMap, prefix: &str, live: &[String]) {
    let stale: alloc::vec::Vec<String> = map
        .tagged_paths()
        .into_iter()
        .filter(|p| p.starts_with(prefix) && !live.iter().any(|l| l == p))
        .map(String::from)
        .collect();
    for path in stale {
        map.drop_path(&path);
    }
}
