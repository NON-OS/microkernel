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

//! The half of the memory pair that holds a secret.
//!
//! It maps a page at an agreed address, fills it with a known pattern, says
//! where, and stays alive long enough for its sibling to go looking.

use nonos_linux_guests::sys::{
    call, out, GETPID, MAP_FIXED, MAP_PRIVATE_ANON, MMAP, NANOSLEEP, PATTERN, PATTERN_AT, PROT_RW,
};

/// Long enough for the reader to run beside it, short enough to be reaped.
const HOLD_SECS: u64 = 60;

fn main() {
    let flags = MAP_PRIVATE_ANON | MAP_FIXED;
    let at = call(MMAP, [PATTERN_AT, 4096, PROT_RW, flags, u64::MAX, 0]);
    if at != PATTERN_AT as i64 {
        out(format!("[GUEST] holder could not map its page: {at}\n").as_bytes());
        return;
    }
    // SAFETY: the page at PATTERN_AT was just mapped read-write, 4096 bytes.
    let page = unsafe { core::slice::from_raw_parts_mut(PATTERN_AT as *mut u8, 4096) };
    for chunk in page.chunks_mut(PATTERN.len()) {
        chunk.copy_from_slice(&PATTERN[..chunk.len()]);
    }
    nonos_linux_guests::shared_name::leave();
    let pid = call(GETPID, [0; 6]);
    out(format!("[GUEST] holder pid={pid} pattern at {PATTERN_AT:#x}\n").as_bytes());
    let ts = [HOLD_SECS, 0u64];
    let _ = call(NANOSLEEP, [ts.as_ptr() as u64, 0, 0, 0, 0, 0]);
    out(b"[GUEST] holder done\n");
}
