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

/* A model's metadata by path: /models is a read-only directory. */

use nonos_libc::mk_data_stat;

use super::name::{owns, volume_name, ROOT};

pub const S_IFDIR: u32 = 0o040000;
pub const S_IFREG: u32 = 0o100000;

/*
 * The mode and size at `path`, None outside /models. Nothing is imported
 * here: a pinned model comes onto the volume when it is first opened.
 */
pub fn stat(path: &[u8]) -> Option<Result<(u32, u64), i64>> {
    if !owns(path) {
        return None;
    }
    if path == ROOT {
        return Some(Ok((S_IFDIR | 0o555, 0)));
    }
    if path == super::catalog::TIERS {
        return Some(Ok((S_IFREG | 0o444, super::catalog::text().len() as u64)));
    }
    let Some(name) = volume_name(path) else {
        return Some(Err(crate::linux::abi::errno::ENOENT));
    };
    let got = mk_data_stat(name);
    Some(if got < 0 { Err(-got) } else { Ok((S_IFREG | 0o444, got as u64)) })
}
