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

//! What the person is told before a pinned model is brought onto the volume.

use alloc::format;

use crate::linux::console::say;

/// Before the import: the file, by its name on the volume, and its size.
pub(in super::super) fn before(name: &[u8], bytes: u64) {
    let file = core::str::from_utf8(name.strip_prefix(b"/").unwrap_or(name)).unwrap_or("model");
    let size = match bytes {
        b if b >= 1_000_000_000 => format!("{}.{} GB", b / 1_000_000_000, b / 100_000_000 % 10),
        b => format!("{} MB", b.div_ceil(1_000_000)),
    };
    /*
     * Said before the import is asked for, when it is not yet known whether
     * the disk carries the file at all: most often it does not, and the
     * next line says how to fetch it. So nothing here claims it is sealing.
     */
    say(format!(
        "qwen: {file} ({size}) is not on the data volume yet; if this disk carries it, it is \
         sealed into the encrypted volume (in memory for this session on a live boot) and \
         checked against its signed SHA-256 pin now, once\n"
    )
    .as_bytes());
}
