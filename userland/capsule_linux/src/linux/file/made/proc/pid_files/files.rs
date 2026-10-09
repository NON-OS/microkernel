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

/* The files in /proc/<pid>/ that are text, and a process's usage. */

use alloc::vec::Vec;
use nonos_libc::peer::mk_peer_read;

use super::super::super::super::cpu::{self, Usage};
use super::super::super::view::Proc;
use super::super::{maps, mounts, pid_status};
use super::stat::stat;
use super::statm::statm;

pub fn content(p: &Proc, tid: u32, file: &[u8]) -> Option<Vec<u8>> {
    Some(match file {
        b"stat" => stat(p, tid),
        b"statm" => statm(p),
        b"status" => pid_status::status(p, tid),
        b"cmdline" => memory(p, p.exe.args, p.exe.env),
        b"environ" => memory(p, p.exe.env, p.exe.end),
        b"comm" => [&p.exe.comm[..], b"\n"].concat(),
        b"limits" => pid_status::limits(),
        b"maps" => maps::maps(p),
        b"mounts" => mounts::mounts(),
        b"mountinfo" => mounts::mountinfo(),
        /* One family, one cgroup: the root of its own hierarchy, as v2 says it. */
        b"cgroup" => b"0::/\n".to_vec(),
        _ => return None,
    })
}

/*
 * The bytes of the process's own memory from `from` to `to`, where the
 * image put argv and the environment. Empty if the record has none.
 */
fn memory(p: &Proc, from: u64, to: u64) -> Vec<u8> {
    let len = to.saturating_sub(from) as usize;
    let mut out = alloc::vec![0u8; len.min(64 << 10)];
    if len == 0 || mk_peer_read(p.kernel, from, &mut out) < 0 {
        return Vec::new();
    }
    out
}

/* The whole process's measured use, or one thread's. */
pub fn usage(p: &Proc, tid: u32) -> Usage {
    let all: Vec<u32> = match p.tids.iter().find(|(ns, _)| *ns == tid && tid != p.ns) {
        Some((_, k)) => alloc::vec![*k],
        None => cpu::threads_of(&[p]),
    };
    let mut u = cpu::usage(&all);
    /* Threads share one address space: its resident size is the leader's. */
    u.resident_kb = cpu::usage(&[p.kernel]).resident_kb;
    u
}

pub fn vsize(p: &Proc) -> u64 {
    p.regions.iter().map(|r| r.len).sum()
}
