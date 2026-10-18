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

//! What the person is told once it is: verified, or why it was refused.

use alloc::format;

use crate::linux::abi::errno;
use crate::linux::console::say;

const EBADMSG: i64 = 74;

/// After the import: verified, or the refusal by name.
pub(in super::super) fn after(done: i64, tier: &str) {
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
