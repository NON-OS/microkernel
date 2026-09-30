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

//! What the person at a terminal is told while a pinned model is brought
//! onto the volume the first time it is used: which file, how large, and
//! then that it was verified or why it was refused. Only the pinned file's
//! name and numbers, said on the terminal only (`console::say`).

use alloc::format;

use crate::linux::abi::errno;
use crate::linux::console::say;

const EBADMSG: i64 = 74;

/// Before the import: the file, by its name on the volume, and its size.
pub(super) fn before(name: &[u8], bytes: u64) {
    let file = core::str::from_utf8(name.strip_prefix(b"/").unwrap_or(name)).unwrap_or("model");
    let size = match bytes {
        b if b >= 1_000_000_000 => format!("{}.{} GB", b / 1_000_000_000, b / 100_000_000 % 10),
        b => format!("{} MB", b.div_ceil(1_000_000)),
    };
    say(format!(
        "qwen: first use of {file} ({size}): sealing it into the encrypted volume and \
         checking its SHA-256 against the signed pin; this happens once\n"
    )
    .as_bytes());
}

/// After the import: verified, or the refusal by name.
pub(super) fn after(done: i64) {
    if done >= 0 {
        say(b"qwen: verified\n");
        return;
    }
    let (name, why) = match -done {
        EBADMSG => ("EBADMSG", "its SHA-256 is not the signed pin's"),
        errno::ENOENT => ("ENOENT", "the disk holds no such file to bring in"),
        errno::EEXIST => ("EEXIST", "the name is taken or the volume's directory is full"),
        errno::ENOMEM => ("ENOMEM", "the volume is full"),
        errno::EPERM => ("EPERM", "this personality may not write the volume"),
        errno::EIO => ("EIO", "the volume or the disk under it failed"),
        _ => ("", ""),
    };
    let line = match name {
        "" => format!("qwen: refused, errno {}\n", -done),
        name => format!("qwen: refused ({name}): {why}\n"),
    };
    say(line.as_bytes());
}
