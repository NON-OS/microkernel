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

//! Host stand-in for libc's syscall bridge, under capsule_load, capsule_verify and lookup.
//
// Replaces libc's syscall/bridge/call_raw.rs (the SYSCALL trampoline), with
// the numbers copied from numbers/core.rs and numbers/ipc.rs over the real
// tag4 (mounted by syscall.rs). call_raw hands each number to its stand-in:
// MCLD to syscall.rs's load recorder, MCVF to verify.rs, MSVL to
// services.rs. Any other number has none and fails the test.

#[path = "syscall.rs"]
mod load;

pub use load::{reset, take, N_MK_CAPSULE_LOAD};

pub const N_MK_CAPSULE_VERIFY: i64 = load::tag::tag4(b"MCVF");
pub const N_MK_SERVICE_LOOKUP: i64 = load::tag::tag4(b"MSVL");

pub fn call_raw(num: i64, args: [u64; 6]) -> i64 {
    match num {
        N_MK_CAPSULE_LOAD => load::call_raw(num, args),
        N_MK_CAPSULE_VERIFY => crate::verify::call(args[0], args[1]),
        N_MK_SERVICE_LOOKUP => crate::services::call(args),
        _ => panic!("syscall {num:#x} has no host stand-in"),
    }
}
