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

//! /sys/devices/system/cpu: which CPUs are online and possible, as
//! "0" or "0-N", read by C runtimes and thread pools sizing themselves.
//! The count is the one sched_getaffinity gives (`machine::cpus`).

use alloc::vec::Vec;

use super::synth::Node;

const CPU: [&[u8]; 2] = [b"system", b"cpu"];
const FILES: [&[u8]; 2] = [b"online", b"possible"];

/// The node at /sys/devices/`rest`.
pub(super) fn node(rest: &[&[u8]]) -> Option<Node> {
    let walked = rest.len().min(CPU.len());
    if rest[..walked] != CPU[..walked] {
        return None;
    }
    match rest.get(CPU.len()..) {
        None => Some(Node::Dir(alloc::vec![CPU[walked].to_vec()])),
        Some([]) => Some(Node::Dir(FILES.iter().map(|f| f.to_vec()).collect())),
        Some([file]) if FILES.contains(file) => Some(Node::Text(0o444)),
        Some(_) => None,
    }
}

/// The bytes of /sys/devices/system/cpu/online or possible.
pub(super) fn content(path: &[u8]) -> Option<Vec<u8>> {
    let file = path.strip_prefix(b"/sys/devices/system/cpu/")?;
    if !FILES.contains(&file) {
        return None;
    }
    let last = crate::linux::file::machine::cpus().saturating_sub(1);
    Some(match last {
        0 => b"0\n".to_vec(),
        n => alloc::format!("0-{n}\n").into_bytes(),
    })
}
