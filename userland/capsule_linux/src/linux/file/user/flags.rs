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

//! A guest's open flags as the broker's (nonos_user_files::wire::flags).

use nonos_user_files::wire::flags::{APPEND, CREATE, DIRECTORY, EXCL, READ, TRUNC, WRITE};

use super::super::flags::{O_APPEND, O_CREAT, O_DIRECTORY, O_EXCL, O_RDWR, O_TRUNC, O_WRONLY};

/// The broker's flags for Linux `flags`. O_ACCMODE 3, which Linux keeps for
/// ioctl-only opens, asks for both, as O_RDWR does.
pub fn of(flags: u64) -> u32 {
    let access = match flags & (O_WRONLY | O_RDWR) {
        0 => READ,
        O_WRONLY => WRITE,
        _ => READ | WRITE,
    };
    let bits = [
        (O_CREAT, CREATE),
        (O_EXCL, EXCL),
        (O_TRUNC, TRUNC),
        (O_APPEND, APPEND),
        (O_DIRECTORY, DIRECTORY),
    ];
    bits.iter().filter(|(linux, _)| flags & linux != 0).fold(access, |f, (_, ours)| f | ours)
}
