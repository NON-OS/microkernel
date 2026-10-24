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

//! The join: every process the kernel's table lists (`MkProcStat`) beside
//! its entry in the attestation registry (`MkAttestEntries`), which holds
//! the mask and the signer the spawn gate recorded from its signed
//! manifest. A capsule in either list is a row; none is dropped. A process
//! the registry does not list has no signed ceiling, and says so.

use alloc::vec::Vec;

/// Registry entry bytes: pid, measurement, capability mask, authority.
pub const ENTRY_LEN: usize = 45;

pub struct Row {
    pub pid: u32,
    /// None when the process table no longer lists the pid.
    pub name: Option<Vec<u8>>,
    /// The registry's facts: mask, signer byte, measurement's first bytes.
    pub ceiling: Option<Ceiling>,
}

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

/// `entries` as the kernel wrote them; `procs` as (pid, name). Pid 0, the
/// kernel's own, is not a capsule. Ordered by pid.
pub fn join(entries: &[u8], procs: &[(u32, &[u8])]) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    for e in entries.chunks_exact(ENTRY_LEN) {
        let (pid, c) = ceiling(e);
        let name = procs.iter().find(|p| p.0 == pid).map(|p| Vec::from(p.1));
        rows.push(Row { pid, name, ceiling: Some(c) });
    }
    for &(pid, name) in procs {
        if pid != 0 && !rows.iter().any(|r| r.pid == pid) {
            rows.push(Row { pid, name: Some(Vec::from(name)), ceiling: None });
        }
    }
    rows.sort_by_key(|r| r.pid);
    rows
}
