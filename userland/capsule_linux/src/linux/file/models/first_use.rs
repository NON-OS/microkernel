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

/// After the import: verified, or the refusal by name.
pub(super) fn after(done: i64, tier: &str) {
    if done >= 0 {
        say(b"qwen: verified\n");
        return;
    }
    /* No disk plan carries it, as on an installed machine: the tier was
     * chosen but never fetched. Say how to fetch it, not only why. */
    if -done == errno::ENOENT {
        say(format!(
            "qwen: the {tier} tier is not on this machine yet. `qwen get {tier}` fetches it \
             over Anyone and checks it against its pin; then ask again.\n"
        )
        .as_bytes());
        return;
    }
    let (name, why) = match -done {
        EBADMSG => ("EBADMSG", "its SHA-256 is not the signed pin's"),
        errno::ENOENT => ("ENOENT", "the disk holds no such file to bring in"),
        errno::EEXIST => ("EEXIST", "the name is taken or the volume's directory is full"),
        errno::ENOSPC => (
            "ENOSPC",
            "the data volume has no room left for it; on a live session it is held in memory, \
             so close programs to free memory and try again, or install NONOS",
        ),
        errno::ENOMEM => (
            "ENOMEM",
            "too little free memory to hold this live session's data volume; install NONOS",
        ),
        errno::EPERM => ("EPERM", "this personality may not write the volume"),
        errno::EIO => {
            ("EIO", "the volume could not be read or written; check the disk and try again")
        }
        errno::EAGAIN => ("EAGAIN", "the disk is not ready yet; try again"),
        errno::EACCES => ("EACCES", "the volume is locked; unlock it with its passphrase"),
        errno::ENODEV => {
            ("ENODEV", "no disk carries NONOS, so there is no data volume: install NONOS")
        }
        _ => ("", ""),
    };
    let line = match name {
        "" => format!("qwen: refused, errno {}\n", -done),
        name => format!("qwen: refused ({name}): {why}\n"),
    };
    say(line.as_bytes());
}
