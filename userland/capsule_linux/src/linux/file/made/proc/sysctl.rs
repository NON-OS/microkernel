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
 * /proc/sys: the few settings programs read, each from the declared surface.
 *
 * All are read-only to a guest: a family cannot rename the system or
 * change how it is run, so each file's mode says so, and a write is
 * refused at open.
 */

use alloc::vec::Vec;

use super::super::super::declared as d;
use super::super::synth::{num, Node};

type Made = fn() -> Vec<u8>;

const FILES: [(&[u8], Made); 8] = [
    (b"fs/pipe-max-size", || line(num(d::PIPE_MAX))),
    (b"kernel/hostname", || line(d::HOSTNAME.to_vec())),
    (b"kernel/osrelease", || line(d::RELEASE.to_vec())),
    (b"kernel/ostype", || line(d::OSTYPE.to_vec())),
    (b"kernel/pid_max", || line(num(d::PID_MAX))),
    (b"kernel/random/boot_id", || line(super::super::boot_id::boot_id())),
    (b"kernel/random/uuid", || line(super::super::boot_id::uuid())),
    (b"vm/overcommit_memory", || line(num(d::OVERCOMMIT))),
];

fn line(mut v: Vec<u8>) -> Vec<u8> {
    v.push(b'\n');
    v
}

fn joined(path: &[&[u8]]) -> Vec<u8> {
    path.join(&b'/')
}

pub fn node(path: &[&[u8]]) -> Option<Node> {
    let at = joined(path);
    if FILES.iter().any(|(p, _)| *p == &at[..]) {
        return Some(Node::Text(0o444));
    }
    let prefix = if at.is_empty() { at.clone() } else { [&at[..], b"/"].concat() };
    let mut names: Vec<Vec<u8>> = Vec::new();
    for (p, _) in FILES.iter() {
        let Some(rest) = p.strip_prefix(&prefix[..]) else { continue };
        let first = rest.split(|b| *b == b'/').next().unwrap_or(rest).to_vec();
        if !names.contains(&first) {
            names.push(first);
        }
    }
    (!names.is_empty()).then_some(Node::Dir(names))
}

pub fn content(path: &[&[u8]]) -> Option<Vec<u8>> {
    let at = joined(path);
    FILES.iter().find(|(p, _)| *p == &at[..]).map(|(_, made)| made())
}
