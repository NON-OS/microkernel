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

use nonos_libc::mk_debug;

/// The step about to run, printed before it. A step that returns an error is
/// reported by the bring-up loop in main once it gives up; a step that never
/// returns reports nothing there, and the last line printed here names it.
pub(super) fn step(name: &str) {
    let mut line = [0u8; 64];
    let tag = b"[BLK] step ";
    let n = tag.len();
    line[..n].copy_from_slice(tag);
    let m = name.len().min(line.len() - n);
    line[n..n + m].copy_from_slice(&name.as_bytes()[..m]);
    let _ = mk_debug(line.as_ptr(), n + m);
}
