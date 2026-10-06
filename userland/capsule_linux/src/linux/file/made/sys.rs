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
 * /sys: only what programs are known to read, and nothing else.
 *
 * Go reads the huge-page size at start to size its heap arenas; C runtimes
 * and thread pools read which CPUs are online (`sys_cpu`). Every other
 * path answers ENOENT, which is what a program meets on a Linux with no
 * sysfs mounted, and which every sysfs reader already handles.
 */

use alloc::vec::Vec;

use super::super::declared;
use super::synth::{num, Node};
use super::sys_cpu;

const THP: [&[u8]; 4] = [b"kernel", b"mm", b"transparent_hugepage", b"hpage_pmd_size"];

pub fn node(rest: &[&[u8]]) -> Option<Node> {
    match rest.split_first() {
        None => Some(Node::Dir(alloc::vec![b"devices".to_vec(), THP[0].to_vec()])),
        Some((&b"devices", more)) => sys_cpu::node(more),
        Some(_) => huge_page(rest),
    }
}

fn huge_page(rest: &[&[u8]]) -> Option<Node> {
    if rest.len() > THP.len() || rest.iter().zip(THP.iter()).any(|(a, b)| a != b) {
        return None;
    }
    Some(match THP.get(rest.len()) {
        Some(next) => Node::Dir(alloc::vec![next.to_vec()]),
        None => Node::Text(0o444),
    })
}

pub fn content(path: &[u8]) -> Option<Vec<u8>> {
    let want = b"/sys/kernel/mm/transparent_hugepage/hpage_pmd_size";
    if path != want {
        return sys_cpu::content(path);
    }
    let mut out = num(declared::HPAGE_PMD);
    out.push(b'\n');
    Some(out)
}
