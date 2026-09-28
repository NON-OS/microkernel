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

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_uptime_ms, mk_yield, Deadline};

use super::start::say;

// The VFS always settles, loaded or given up; this bound only covers a dead one.
const READY_MS: u64 = 300_000;

/*
 * Wait until the VFS has finished loading the store, so a missing file is
 * really missing. The time it took is logged as a number: the bound is a
 * liveness backstop, and a slower settle must show up rather than hide under it.
 */
pub(super) fn wait_settled() -> bool {
    let start = mk_uptime_ms();
    let until = Deadline::after_ms(READY_MS);
    while !matches!(vfs::store_settled(), Ok(true)) {
        if until.expired() {
            say(b"[LINUX] boot guest unreadable: store never settled\n");
            return false;
        }
        let _ = mk_yield();
    }
    const PREFIX: &[u8] = b"[LINUX] store settled after ";
    // Room for the prefix, 20 digits and " ms\n".
    let mut line = [0u8; 52];
    line[..PREFIX.len()].copy_from_slice(PREFIX);
    let ms = mk_uptime_ms().saturating_sub(start).max(0) as u64;
    let n = write_ms(&mut line[PREFIX.len()..], ms);
    super::start::note(&line[..PREFIX.len() + n]);
    true
}

// "<n> ms\n" into `out`, returning the bytes written.
fn write_ms(out: &mut [u8], mut ms: u64) -> usize {
    let mut digits = [0u8; 20];
    let mut len = 0;
    loop {
        digits[len] = b'0' + (ms % 10) as u8;
        len += 1;
        ms /= 10;
        if ms == 0 {
            break;
        }
    }
    for i in 0..len {
        out[i] = digits[len - 1 - i];
    }
    out[len..len + 4].copy_from_slice(b" ms\n");
    len + 4
}
