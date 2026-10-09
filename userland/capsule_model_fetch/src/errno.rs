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

/* A refusal by name, and what it means for a download. */

const NAMES: &[(i64, &str, &str)] = &[
    (-1, "EPERM", "this program may not feed the data volume"),
    (-2, "ENOENT", "the data volume holds no such file"),
    (
        -5,
        "EIO",
        "the data volume could not be read or written; check the disk and try again \
         (with no NONOS disk at all there is nowhere to keep models)",
    ),
    (-11, "EAGAIN", "the disk is not ready yet; try again"),
    (
        -12,
        "ENOMEM",
        "too little free memory to hold this live session's data volume; install NONOS \
         to keep models on a disk",
    ),
    (-13, "EACCES", "the data volume is locked; unlock it with its passphrase first"),
    (-16, "EBUSY", "another download is feeding the data volume"),
    (-17, "EEXIST", "the name is taken on the data volume by another file"),
    (
        -19,
        "ENODEV",
        "no disk carries NONOS, so there is no data volume: install NONOS to keep models",
    ),
    (-22, "EINVAL", "the kernel refused the request"),
    (-27, "EFBIG", "the mirror sent more than the pinned length"),
    (
        -28,
        "ENOSPC",
        "the data volume has no room for it; on a live session it is held in memory, so \
         close programs to free memory and try again, or install NONOS",
    ),
    (-74, "EBADMSG", "its SHA-256 is not the signed pin; the kernel discarded it"),
    (-115, "EINPROGRESS", "the file is not whole yet"),
];

/*
 * The volume has no room for the `left` bytes still to come: on a live
 * boot (`memory`) the volume is this session's memory, on an installed
 * NONOS the disk.
 */
pub fn no_room(memory: bool, left: u64) -> alloc::string::String {
    match memory {
        true => alloc::format!(
            "this live session holds its data volume in memory, and there is not enough free \
             for the {} still to come; close programs and try again, or install NONOS to keep \
             models on a disk (ENOSPC)",
            crate::size::size(left)
        ),
        false => alloc::format!(
            "the data volume on the disk has no room for the {} still to come; uninstall a \
             tier to make room (ENOSPC)",
            crate::size::size(left)
        ),
    }
}

/* "EBADMSG: its SHA-256 is not ...", or the number for one not named here. */
pub fn said(rc: i64) -> alloc::string::String {
    match NAMES.iter().find(|(e, _, _)| *e == rc) {
        Some((_, name, why)) => alloc::format!("{why} ({name})"),
        None => alloc::format!("the kernel refused (errno {})", -rc),
    }
}
