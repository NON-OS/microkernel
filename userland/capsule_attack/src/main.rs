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

//! The attack suite's capsule. Test only: the kernel embeds it under
//! `nonos-capsule-attack`, which the selftest profile alone turns on and which
//! no release can carry (src/userspace/capsule_attack).
//!
//! Each attempt is one the capsule holds no right to, and each prints one line
//! once the kernel has answered: `[ATTACK] <name> refused: <how>`, or
//! `[ATTACK] <name> ESCAPED: <what>`.

#![no_std]
#![no_main]

mod attempts;
mod authority;
mod codes;
mod containment;
mod deputy;
mod deputy_port;
mod disks;
mod errno;
mod line;
mod medium;
mod network;
mod reply_caller;
mod reply_escape;
mod reply_seen;
mod reply_take;
mod reply_twice;

/// # Safety
/// The kernel enters here once, on a fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // First, so the checker pairs each refusal with the kernel's own
    // [CAP-DENY] line for this pid and no other's.
    line::marker(b"[ATTACK-START] capsule pid ", u64::from(nonos_libc::mk_getpid()), b"\n");
    network::cap_escape();
    attempts::inbox_escape();
    reply_escape::reply_escape();
    reply_twice::reply_twice();
    authority::authority_escape();
    attempts::foreign_memory();
    attempts::register_service();
    medium::raw_disk();
    deputy::deputy_escape();
    attempts::service_squat();
    line::note(b"[ATTACK-DONE] capsule\n");
    // Last, because the second half ends the capsule.
    containment::spin();
    containment::fault()
}
