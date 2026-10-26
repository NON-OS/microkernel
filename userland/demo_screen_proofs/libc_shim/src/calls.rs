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

//! The plain calls: who and when, capabilities, lookups, the display.

use crate::machine::{name_of, MODE, OWN_PID, PROCS};
use crate::user_mem::{bytes, put};

pub fn mk_getpid() -> u32 {
    OWN_PID
}

/// Two hours, fourteen minutes and nine seconds into the boot.
pub fn mk_uptime_ms() -> i64 {
    8_049_000
}

/// 24 October 2026, 10:00 UTC.
pub fn mk_time_millis() -> i64 {
    1_792_836_000_000
}

pub fn mk_cap_check(pid: u32, mask: u64) -> bool {
    PROCS.iter().any(|&(p, _, m)| p == pid && m & mask == mask)
}

/// Every process's name is announced, the shown driver's among them.
pub fn mk_service_lookup(name: *const u8, len: usize, port: *mut u32, pid: *mut u32) -> i64 {
    // SAFETY: the caller hands `len` bytes at `name`, as the real call does.
    let name = unsafe { bytes(name as usize, len) };
    let Some(&(p, _, _)) = PROCS.iter().find(|e| name_of(e.1).as_bytes() == name) else {
        return -2;
    };
    // SAFETY: each out pointer is null or a u32 the caller holds.
    unsafe {
        if !port.is_null() {
            put(port as usize, 4000 + p);
        }
        if !pid.is_null() {
            put(pid as usize, p);
        }
    }
    0
}

/// No service answers a call on the host: the proof board reads as unknown.
pub fn mk_ipc_call_timeout(_: u64, _: *const u8, _: usize, _: *mut u8, _: usize, _: u64) -> i64 {
    -110
}

pub fn nonos_display_dimensions(display: u32, w: *mut u32, h: *mut u32) -> i64 {
    if display != 0 || w.is_null() || h.is_null() {
        return -22;
    }
    let (mw, mh) = MODE.with(|m| m.get());
    // SAFETY: both out pointers were checked non-null and are the caller's.
    unsafe { (put(w as usize, mw), put(h as usize, mh)) };
    0
}
