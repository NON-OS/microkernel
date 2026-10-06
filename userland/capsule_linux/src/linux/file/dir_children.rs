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

//! The names directly below a directory, from the store's flat listing.

use alloc::string::String;
use alloc::vec::Vec;

// OP_LIST returns whole keys at any depth. Cut at the first separator
// past the prefix and dedupe, or every file below shows up as a sibling.
pub fn children(at: &[u8], keys: Vec<String>) -> Vec<String> {
    let cut = at.len() + 1;
    let mut out: Vec<String> = Vec::new();
    for key in keys {
        let bytes = key.as_bytes();
        // The listing matches bytes, so `/linux` also returns `/linux-deb/..`:
        // a key is below `at` only when a separator follows it.
        if bytes.len() <= cut || bytes.get(at.len()) != Some(&b'/') {
            continue;
        }
        let rest = &bytes[cut..];
        let end = rest.iter().position(|b| *b == b'/').unwrap_or(rest.len());
        let Ok(name) = core::str::from_utf8(&rest[..end]) else {
            continue;
        };
        if !out.iter().any(|seen| seen == name) {
            out.push(String::from(name));
        }
    }
    out
}
