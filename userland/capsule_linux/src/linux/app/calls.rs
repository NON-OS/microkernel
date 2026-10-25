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

//! The declared call set, enforced before a call is served: a call whose
//! bucket the policy leaves clear is EPERM, so it lands on the failed side
//! of the kernel's count and the isolation proof shows the box held.

use core::sync::atomic::{AtomicU8, Ordering};

use nonos_linux_policy::{bucket, SET_LEN};

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// Buckets already said once, so a program that retries a refused call in
/// a loop does not bury the log.
static SAID: [AtomicU8; SET_LEN] = [const { AtomicU8::new(0) }; SET_LEN];

/// Whether call `number` is outside what the guest may make, said nowhere.
pub fn undeclared(guest: &Guest, number: u64) -> bool {
    super::rules::call_refused(&guest.app.calls, number)
}

/// The answer for call `number` when the guest may not make it
/// (rules::call_refused).
pub fn outside(guest: &Guest, number: u64) -> Option<u64> {
    if !super::rules::call_refused(&guest.app.calls, number) {
        return None;
    }
    let tag = alloc::format!(
        "call {number} {}",
        alloc::string::String::from_utf8_lossy(crate::linux::abi::name::of(number),)
    );
    crate::linux::boxed::note(crate::linux::boxed::Kind::Call, tag.as_bytes());
    let b = bucket(number);
    let bit = 1u8 << (b % 8);
    if SAID[b / 8].fetch_or(bit, Ordering::Relaxed) & bit == 0 {
        let line = alloc::format!("[LINUX] refused call {number}: outside its declared calls\n");
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
    Some(errno::fail(errno::EPERM))
}
