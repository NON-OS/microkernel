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

//! Starting the model fetcher with a verb and a tier, and waiting for its
//! exit while what it writes is drained and dropped (fetcher.rs says why).

use alloc::vec::Vec;

use nonos_libc::{mk_proc_output, mk_tool_run, mk_wait};

const TOOL: &[u8] = b"tool.model-fetch";
const ETIMEDOUT: i64 = -110;
/* How long one wait lasts before the fetcher's output is drained again. */
const SLICE_MS: u64 = 200;
/* The most the fetcher writes at once. */
const LINE: usize = 256;

pub(super) fn run(verb: &[u8], tier: &str) -> Result<i64, i64> {
    let mut argv = Vec::with_capacity(verb.len() + 1 + tier.len());
    argv.extend_from_slice(verb);
    argv.push(0);
    argv.extend_from_slice(tier.as_bytes());
    let rc = mk_tool_run(TOOL, &argv);
    let pid = match u32::try_from(rc) {
        Ok(pid) if pid != 0 => pid,
        _ => return Err(rc.min(-1)),
    };
    loop {
        drain(pid);
        let status = mk_wait(u64::from(pid), SLICE_MS);
        if status != ETIMEDOUT {
            drain(pid);
            return Ok(status);
        }
    }
}

/* Read and drop what the fetcher has written so far. */
fn drain(pid: u32) {
    let mut line = [0u8; LINE];
    while mk_proc_output(pid, line.as_mut_ptr(), line.len()) > 0 {}
}
