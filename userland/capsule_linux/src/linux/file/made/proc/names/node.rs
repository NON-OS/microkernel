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

/* The node a parsed /proc path is, for the asking process's view. */

use alloc::vec::Vec;

use super::super::super::synth::{num, Node};
use super::super::super::view::{Proc, View};
use super::at::At;
use super::lists::{DIRS, FILES, LINKS, SYSTEM};
use super::parse::parse;

pub fn node(v: &View, rest: &[&[u8]]) -> Option<Node> {
    Some(match parse(v, rest)? {
        At::Root => Node::Dir(root(v)),
        At::Sysctl(path) => super::super::sysctl::node(path)?,
        At::PidDir(tid) => Node::Dir(pid_names(tid.is_none())),
        At::Task(p) => Node::Dir(p.tids.iter().map(|(ns, _)| num(u64::from(*ns))).collect()),
        At::FdDir(p) | At::FdInfoDir(p) => {
            Node::Dir(p.fds.iter().map(|f| num(u64::from(f.fd))).collect())
        }
        At::Fd { proc, fd } => Node::Link(super::super::fds::target_of(proc, fd)?),
        At::Link(to) => Node::Link(to),
        At::Pid { file: b"mem", .. } => Node::Refused("/proc/<pid>/mem: a second way into memory"),
        At::Pid { file: b"environ", .. } => Node::Text(0o400),
        _ => Node::Text(0o444),
    })
}

fn root(v: &View) -> Vec<Vec<u8>> {
    let mut names: Vec<Vec<u8>> = v.procs.iter().map(|p| num(u64::from(p.ns))).collect();
    names.extend(SYSTEM.iter().map(|s| s.to_vec()));
    names.extend([&b"mounts"[..], b"self", b"sys", b"thread-self"].iter().map(|s| s.to_vec()));
    names
}

fn pid_names(leader: bool) -> Vec<Vec<u8>> {
    let dirs = DIRS.iter().filter(|d| leader || **d != b"task");
    FILES.iter().chain(LINKS.iter()).chain(dirs).map(|n| n.to_vec()).collect()
}

pub(super) fn open_fd(proc: &Proc, n: &[u8]) -> Option<u32> {
    let fd = number(n)?;
    proc.fds.iter().any(|f| f.fd == fd).then_some(fd)
}

/* A decimal with no sign and no leading zero, as /proc names are. */
pub fn number(s: &[u8]) -> Option<u32> {
    if s.is_empty() || (s[0] == b'0' && s.len() > 1) || !s.iter().all(u8::is_ascii_digit) {
        return None;
    }
    core::str::from_utf8(s).ok()?.parse().ok()
}
