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

//! The terminal's answers to what a shared builtin asks of the machine, from
//! the kernel under the terminal's own capabilities: randomness (Crypto), the
//! clocks, memory from the process table's header, and files
//! through the VFS as the terminal's owner, relative to its working
//! directory.

use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::read_file;
use nonos_shell::host::{Host, Mem, Proc};

use super::builtin::proc_table::table;
use crate::term::cwd::resolve;

pub struct TermHost<'a> {
    pub cwd: &'a [u8],
    pub owner_pid: u32,
}

impl Host for TermHost<'_> {
    fn random(&mut self, buf: &mut [u8]) -> bool {
        let rc = nonos_libc::crypto_random(buf.as_mut_ptr(), buf.len());
        usize::try_from(rc).ok() == Some(buf.len())
    }

    fn wall_ms(&mut self) -> Option<u64> {
        u64::try_from(nonos_libc::mk_time_millis()).ok().filter(|ms| *ms > 0)
    }

    fn uptime_ms(&mut self) -> Option<u64> {
        u64::try_from(nonos_libc::mk_uptime_ms()).ok()
    }

    fn mem(&mut self) -> Option<Mem> {
        let (total, free) = nonos_libc::mem_kb()?;
        Some(Mem { total: total.checked_mul(1024)?, used: (total - free).checked_mul(1024)? })
    }

    fn procs(&mut self) -> Option<Vec<Proc>> {
        let rows = table().into_iter().map(|p| Proc {
            pid: p.pid,
            ppid: p.ppid,
            state: p.state,
            name: p.name_str().as_bytes().to_vec(),
        });
        Some(rows.collect())
    }

    fn read_file(&mut self, path: &[u8], max: usize) -> Option<Vec<u8>> {
        let cap = u32::try_from(max.checked_add(1)?).ok()?;
        let data = read_file(self.owner_pid, &resolve(self.cwd, path), cap).ok()?;
        (data.len() <= max).then_some(data)
    }
}
