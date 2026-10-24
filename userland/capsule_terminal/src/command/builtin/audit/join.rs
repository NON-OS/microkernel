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

//! Every process the kernel lists (`MkProcStat`) beside its registry entry
//! (`MkAttestEntries`): the mask and signer recorded from its signed
//! manifest. A capsule in either is one row; one with no entry says so.

use alloc::vec::Vec;

/// Registry entry bytes: pid, measurement, capability mask, authority.
pub const ENTRY_LEN: usize = 45;

pub struct Row {
    pub pid: u32,
    /// None once the process table no longer lists the pid.
    pub name: Option<Vec<u8>>,
    /// The registry's facts: mask, signer byte, measurement's first bytes.
    pub ceiling: Option<Ceiling>,
    /// The process table says the spawn gate withheld Debug (`withheld`).
    pub debug_withheld: bool,
}

/// A process as the kernel's table lists it: pid, name, Debug withheld.
pub type Proc<'a> = (u32, &'a [u8], bool);

#[derive(Clone, Copy)]
pub struct Ceiling {
    pub caps: u64,
    pub authority: u8,
    pub measurement: [u8; 6],
}

fn ceiling(e: &[u8]) -> (u32, Ceiling) {
    let pid = u32::from_be_bytes([e[0], e[1], e[2], e[3]]);
    let caps = u64::from_be_bytes(e[36..44].try_into().unwrap_or([0; 8]));
    let mut measurement = [0u8; 6];
    measurement.copy_from_slice(&e[4..10]);
    (pid, Ceiling { caps, authority: e[44], measurement })
}

/// `entries` as the kernel wrote them; `procs` as listed. Pid 0, the
/// kernel's own, is not a capsule. Ordered by pid. None for a registry that
/// is not whole entries, which this build and the kernel disagree on.
pub fn join(entries: &[u8], procs: &[Proc<'_>]) -> Option<Vec<Row>> {
    if !entries.len().is_multiple_of(ENTRY_LEN) {
        return None;
    }
    let mut rows: Vec<Row> = Vec::new();
    for e in entries.chunks_exact(ENTRY_LEN) {
        let (pid, c) = ceiling(e);
        let p = procs.iter().find(|p| p.0 == pid);
        let (name, debug_withheld) = (p.map(|p| Vec::from(p.1)), p.is_some_and(|p| p.2));
        rows.push(Row { pid, name, ceiling: Some(c), debug_withheld });
    }
    for &(pid, name, debug_withheld) in procs {
        if pid != 0 && !rows.iter().any(|r| r.pid == pid) {
            rows.push(Row { pid, name: Some(Vec::from(name)), ceiling: None, debug_withheld });
        }
    }
    rows.sort_by_key(|r| r.pid);
    Some(rows)
}
