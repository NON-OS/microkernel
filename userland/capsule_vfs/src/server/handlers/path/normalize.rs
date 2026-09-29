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
use alloc::vec;

use super::normalize_to_buffer;

/*
 * None when any component is `..`. Every caller that means a parent resolves
 * it before calling, as the terminal and the Linux personality do, so vfs
 * never has to decide what a climb above some caller's root should reach.
 * With the personality's own clamp removed, a guest's `/../capsules` reached
 * the capsule tree through here; this is the second barrier.
 */
pub(crate) fn normalize(path: &str) -> Option<String> {
    if path.split('/').any(|part| part == "..") {
        return None;
    }
    let needed = path.len().saturating_add(1);
    let mut out = vec![0; needed];
    let len = normalize_to_buffer(path.as_bytes(), &mut out);
    out.truncate(len);
    /*
     * SAFETY: `path` is a &str, and normalize_to_buffer only drops whole
     * components between '/' bytes and inserts '/', into a buffer one byte
     * longer than its input, so every multi-byte sequence it copies is whole.
     */
    Some(unsafe { String::from_utf8_unchecked(out) })
}
