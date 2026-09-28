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

/* The metadata for a made node, and for a name the store keeps. */

use super::super::super::made::dev;
use super::super::super::synth::{Node, S_IFCHR, S_IFDIR, S_IFLNK, S_IFREG};
use super::super::statbuf::Meta;
use super::fd::now;
use super::path::{at, S_IFIFO, S_IFSOCK};

pub(super) fn made(path: &[u8], node: Node) -> Result<Meta, i64> {
    let t = now();
    Ok(match node {
        Node::Dir(_) if path.starts_with(b"/dev") => at(path, S_IFDIR | 0o755, 0, t),
        Node::Dir(_) => at(path, S_IFDIR | 0o555, 0, t),
        Node::Text(mode) => at(path, S_IFREG | mode, 0, t),
        Node::Refused(_) => at(path, S_IFREG | 0o600, 0, t),
        Node::Dev(d) => Meta { rdev: dev::rdev(d), ..at(path, S_IFCHR | 0o666, 0, t) },
        /* Followed already, so a link here names no path: a pipe or a socket. */
        Node::Link(to) if to.contains(&b':') => named(&to, path),
        Node::Link(to) => at(path, S_IFLNK | 0o777, to.len() as u64, t),
    })
}

/*
 * What has no path, by the name /proc gives it: `pipe:[n]`, `socket:[n]`,
 * `anon_inode:[...]`.
 */
pub(super) fn named(to: &[u8], path: &[u8]) -> Meta {
    let number = |skip: usize| {
        let digits = to.get(skip..to.len().saturating_sub(1)).unwrap_or(b"");
        core::str::from_utf8(digits).ok().and_then(|s| s.parse().ok()).unwrap_or(0)
    };
    let t = now();
    match to {
        _ if to.starts_with(b"pipe:[") => {
            Meta { ino: number(6), ..at(path, S_IFIFO | 0o600, 0, t) }
        }
        _ if to.starts_with(b"socket:[") => {
            Meta { ino: number(8), ..at(path, S_IFSOCK | 0o777, 0, t) }
        }
        _ => Meta { ino: 0, ..at(path, 0o600, 0, t) },
    }
}
