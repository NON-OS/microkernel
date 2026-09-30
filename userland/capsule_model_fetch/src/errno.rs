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
    (-5, "EIO", "the data volume or the disk under it failed"),
    (-11, "EAGAIN", "no disk is chosen yet"),
    (-12, "ENOMEM", "the data volume is full"),
    (-13, "EACCES", "the data volume is locked; unlock it with its passphrase first"),
    (-16, "EBUSY", "another download is feeding the data volume"),
    (-17, "EEXIST", "the name is taken on the data volume by another file"),
    (-22, "EINVAL", "the kernel refused the request"),
    (-27, "EFBIG", "the mirror sent more than the pinned length"),
    (-28, "ENOSPC", "the data volume has no room for it"),
    (-74, "EBADMSG", "its SHA-256 is not the signed pin; the kernel discarded it"),
    (-115, "EINPROGRESS", "the file is not whole yet"),
];

/* "EBADMSG: its SHA-256 is not ...", or the number for one not named here. */
pub fn said(rc: i64) -> alloc::string::String {
    match NAMES.iter().find(|(e, _, _)| *e == rc) {
        Some((_, name, why)) => alloc::format!("{why} ({name})"),
        None => alloc::format!("the kernel refused (errno {})", -rc),
    }
}
