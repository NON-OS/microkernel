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
 * Handing a tier to the model fetcher, `tool.model-fetch`, as `qwen get
 * TIER` does from the Terminal. The kernel starts it through the verified
 * path, parented to this installer, which holds the network and the files
 * as the Terminal does. The fetcher downloads over the system's chosen
 * network and feeds the kernel, which keeps a file only if its SHA-256 is
 * the pin. Nothing here touches the network or the bytes.
 *
 * What the fetcher says is drained and dropped, never logged: tier names,
 * mirror hosts and progress do not reach the serial log through it, and a
 * line nobody drains would hold it for seconds. Its exit status is the
 * reason (`fetch_exit`).
 */

use alloc::vec::Vec;

use nonos_libc::{mk_proc_output, mk_tool_run, mk_wait};

const TOOL: &[u8] = b"tool.model-fetch";
const VERB: &[u8] = b"get";
const ETIMEDOUT: i64 = -110;
/* How long one wait lasts before the fetcher's output is drained again. */
const SLICE_MS: u64 = 200;
/* The most the fetcher writes at once. */
const LINE: usize = 256;

/*
 * The fetcher's exit status for `get <tier>`, or MkToolRun's errno. With
 * `direct`, `get --direct <tier>`: the person chose a direct download in
 * the store for this install.
 */
pub fn fetch(tier: &str, direct: bool) -> Result<i64, i64> {
    match direct {
        true => run(b"get\0--direct", tier),
        false => run(VERB, tier),
    }
}

/*
 * The fetcher's exit status for `remove <tier>`, which takes the tier's
 * model files off the data volume, or MkToolRun's errno. The fetcher holds
 * StreamImport, the right the kernel asks for that; this installer does not.
 */
pub fn remove(tier: &str) -> Result<i64, i64> {
    run(b"remove", tier)
}

fn run(verb: &[u8], tier: &str) -> Result<i64, i64> {
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
