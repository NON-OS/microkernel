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

/* The device numbers stat reports, and /dev/fd's links. */

use super::super::synth::Node;
use super::tree::{Dev, DEVICES};
use crate::linux::pty;

/* st_rdev, in the encoding glibc's and musl's makedev use. */
pub fn rdev(dev: Dev) -> u64 {
    if let Dev::Pts(n) = dev {
        return 136 << 8 | u64::from(n & 0xff) | u64::from(n & !0xff) << 12;
    }
    let (_, _, major, minor) = DEVICES.iter().find(|d| d.1 == dev).copied().unwrap_or(DEVICES[0]);
    let (major, minor) = (u64::from(major), u64::from(minor));
    ((major & 0xfff) << 8) | (minor & 0xff) | ((minor & !0xff) << 12) | ((major & !0xfff) << 32)
}

/* The device a path names, for a descriptor already open on it. */
pub fn at(path: &[u8]) -> Option<Dev> {
    let name = path.strip_prefix(b"/dev/")?;
    DEVICES.iter().find(|d| d.0 == name).map(|d| d.1)
}

/* /dev/pts: a slave for each pseudo-terminal whose master is open. */
pub(super) fn pts(tail: &[&[u8]]) -> Option<Node> {
    let names = pty::names();
    match tail {
        [] => Some(Node::Dir(names.iter().map(|n| alloc::format!("{n}").into_bytes()).collect())),
        [n] => pty::pts_number(&[b"/dev/pts/", *n].concat())
            .filter(|n| names.contains(n))
            .map(|n| Node::Dev(Dev::Pts(n))),
        _ => None,
    }
}
