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

/*
 * A file's data beyond read and write: the positional and vector forms,
 * copies between descriptors, a file's length, syncing, and the opens
 * with more to say than openat.
 */

use crate::linux::abi::{nr, nr_path as np};
use crate::linux::file::{self, flags::AT_FDCWD, xattr};
use crate::linux::guest::Guest;

/* creat is open with O_CREAT | O_WRONLY | O_TRUNC. */
const CREAT_FLAGS: u64 = 0o1101;

/*
 * The xattr calls' arguments after the path or descriptor: name, value,
 * size, flags; list has only a buffer and its size.
 */
fn args(a: [u64; 6]) -> xattr::Args {
    xattr::Args { name: a[1], value: a[2], size: a[3], flags: a[4] }
}

fn list_args(a: [u64; 6]) -> xattr::Args {
    xattr::Args { name: 0, value: a[1], size: a[2], flags: 0 }
}

pub fn data_ops(guest: &mut Guest, nr: u64, a: [u64; 6]) -> Option<u64> {
    Some(match nr {
        nr::PWRITE64 => file::pwrite64(guest, a[0], a[1], a[2], a[3]),
        np::PREADV => file::preadv(guest, a[0], a[1], a[2], a[3], 0),
        np::PWRITEV => file::pwritev(guest, a[0], a[1], a[2], a[3], 0),
        np::PREADV2 => file::preadv(guest, a[0], a[1], a[2], a[3], a[5]),
        np::PWRITEV2 => file::pwritev(guest, a[0], a[1], a[2], a[3], a[5]),
        np::SENDFILE => file::sendfile(guest, a[0], a[1], a[2], a[3]),
        np::COPY_FILE_RANGE => file::copy_file_range(guest, a),
        np::TRUNCATE => file::truncate(guest, a[0], a[1]),
        np::FALLOCATE => file::fallocate(guest, a[0], a[1], a[2], a[3]),
        np::FADVISE64 => file::fadvise64(guest, a[0], a[3]),
        np::CLOSE_RANGE => file::close_range(guest, a[0], a[1], a[2]),
        np::SYNC => file::sync(),
        np::SYNCFS => file::syncfs(guest, a[0]),
        np::CREAT => file::openat(guest, AT_FDCWD, a[0], CREAT_FLAGS, a[1]),
        np::OPENAT2 => file::openat2(guest, a[0], a[1], a[2], a[3]),
        np::GETXATTR => xattr::by_path(guest, a[0], xattr::Op::Get, args(a), true),
        np::LGETXATTR => xattr::by_path(guest, a[0], xattr::Op::Get, args(a), false),
        np::FGETXATTR => xattr::by_fd(guest, a[0], xattr::Op::Get, args(a)),
        np::SETXATTR => xattr::by_path(guest, a[0], xattr::Op::Set, args(a), true),
        np::LSETXATTR => xattr::by_path(guest, a[0], xattr::Op::Set, args(a), false),
        np::FSETXATTR => xattr::by_fd(guest, a[0], xattr::Op::Set, args(a)),
        np::LISTXATTR => xattr::by_path(guest, a[0], xattr::Op::List, list_args(a), true),
        np::LLISTXATTR => xattr::by_path(guest, a[0], xattr::Op::List, list_args(a), false),
        np::FLISTXATTR => xattr::by_fd(guest, a[0], xattr::Op::List, list_args(a)),
        np::REMOVEXATTR => xattr::by_path(guest, a[0], xattr::Op::Remove, args(a), true),
        np::LREMOVEXATTR => xattr::by_path(guest, a[0], xattr::Op::Remove, args(a), false),
        np::FREMOVEXATTR => xattr::by_fd(guest, a[0], xattr::Op::Remove, args(a)),
        _ => return None,
    })
}
