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

/* The trees the personality makes, and what a path in them is. */

use alloc::vec::Vec;

use super::super::dev::Dev;

pub const S_IFDIR: u32 = 0o040000;

pub const S_IFREG: u32 = 0o100000;

pub const S_IFLNK: u32 = 0o120000;

pub const S_IFCHR: u32 = 0o020000;

pub enum Node {
    /* A directory and the names in it. */
    Dir(Vec<Vec<u8>>),
    /* A file whose bytes are made at each read, and its permission bits. */
    Text(u32),
    /* A symbolic link, as readlink gives it. */
    Link(Vec<u8>),
    Dev(Dev),
    /* There, and refused on purpose: the reason is said when it is opened. */
    Refused(&'static str),
}

/* Which tree a path is in, if any: the store keeps /dev/shm, not /dev. */
pub fn owns(path: &[u8]) -> bool {
    let under =
        |root: &[u8]| path.starts_with(root) && matches!(path.get(root.len()), None | Some(b'/'));
    (under(b"/proc") || under(b"/sys") || under(b"/dev")) && !under(b"/dev/shm")
}

/*
 * The node at `path`: None outside these trees, Err(ENOENT) inside them
 * where there is nothing.
 */
pub fn node(path: &[u8]) -> Option<Result<Node, i64>> {
    if !owns(path) {
        return None;
    }
    let parts: Vec<&[u8]> = path.split(|b| *b == b'/').filter(|p| !p.is_empty()).collect();
    let missing = crate::linux::abi::errno::ENOENT;
    Some(
        match parts.split_first() {
            Some((&b"dev", rest)) => super::super::dev::node(rest),
            Some((&b"sys", rest)) => super::super::sys::node(rest),
            Some((_, rest)) => super::super::proc::node(rest),
            None => None,
        }
        .ok_or(missing),
    )
}
