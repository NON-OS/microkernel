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

//! `capsules`: every capsule running, by name, with the capabilities the
//! kernel granted it.
//!
//! It used to look up a fixed list of 24 service names and print "[live]" or
//! "[absent]" for each: capsules off the list never showed, nothing it printed
//! was a capability, and an absent name read as a failure when the build
//! simply had no such capsule. The names now come from the process table and
//! the capabilities from the attestation registry `receipt` reads.

use alloc::vec::Vec;

use super::cap_names::CAP_NAMES;
use super::proc_table::table;
use super::receipt::own::ENTRY_LEN;
use crate::command::output::Output;
use crate::term::util::format_u64;

pub fn run(out: &mut Output<'_>, _argv: &[&[u8]]) {
    let procs = table();
    let name_of = |pid: u32| procs.iter().find(|p| p.pid == pid).map(|p| p.name_str());
    let regs = match super::receipt::entries() {
        Ok(regs) => regs,
        Err(_) => {
            out.writeln(b"capabilities: the kernel did not hand over its registry");
            for p in procs.iter().filter(|p| p.pid != 0) {
                out.writeln(&row(p.pid, p.name_str().as_bytes(), None));
            }
            return;
        }
    };
    for e in regs.chunks_exact(ENTRY_LEN) {
        let pid = u32::from_be_bytes([e[0], e[1], e[2], e[3]]);
        let caps = u64::from_be_bytes([e[36], e[37], e[38], e[39], e[40], e[41], e[42], e[43]]);
        let name = name_of(pid).unwrap_or("(exited)");
        out.writeln(&row(pid, name.as_bytes(), Some(caps)));
    }
}

/// "  pid  name  Cap, Cap", the capabilities left off when they are unknown.
fn row(pid: u32, name: &[u8], caps: Option<u64>) -> Vec<u8> {
    let mut line = Vec::with_capacity(96);
    let mut num = [0u8; 12];
    let n = format_u64(pid as u64, &mut num);
    line.resize(6usize.saturating_sub(n), b' ');
    line.extend_from_slice(&num[..n]);
    line.extend_from_slice(b"  ");
    line.extend_from_slice(name);
    let Some(caps) = caps else { return line };
    line.resize(line.len().max(34), b' ');
    let mut first = true;
    for (i, cap) in CAP_NAMES.iter().enumerate() {
        if caps & (1u64 << i) != 0 {
            line.extend_from_slice(if first { b"  " } else { b", " });
            line.extend_from_slice(cap);
            first = false;
        }
    }
    line
}
