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
 * A /proc path's node, and a /proc file's bytes, made now from the view the
 * family lent for the call.
 */

use alloc::vec::Vec;

use super::super::synth::Node;
use super::super::view::{self, View};
use super::names::{self, parse, At};
use super::{fds, pid_files, sysctl, system};

pub fn node(rest: &[&[u8]]) -> Option<Node> {
    view::with(|v| names::node(v, rest))
}

/* The bytes of the /proc file at `path`, made now; Err is the errno. */
pub fn content(path: &[u8]) -> Result<Vec<u8>, i64> {
    let parts: Vec<&[u8]> = path.split(|b| *b == b'/').filter(|p| !p.is_empty()).collect();
    let missing = crate::linux::abi::errno::ENOENT;
    let rest = parts.get(1..).ok_or(missing)?;
    view::with(|v| made(v, rest)).ok_or(missing)
}

fn made(v: &View, rest: &[&[u8]]) -> Option<Vec<u8>> {
    match parse(v, rest)? {
        At::System(name) => system::content(v, name),
        At::Sysctl(path) => sysctl::content(path),
        At::Pid { proc, tid, file } => pid_files::content(proc, tid, file),
        At::FdInfo { proc, fd } => fds::info(proc, fd),
        _ => None,
    }
}
