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

//! Sleeping without burning the processor.

use crate::syscall::{call_raw, N_MK_FUTEX_WAIT};

/// Sleep for up to `ms`.
///
/// `mk_yield` is not a sleep. On a machine where nothing else wants the
/// processor it returns immediately, so a retry loop built from yields runs flat
/// out: capsule_login sat at eighty-eight percent of a core for twenty-three
/// minutes doing exactly that, waiting on a step that was never going to arrive
/// sooner for the spinning.
///
/// A futex wait on a private word is the honest primitive. Nobody else knows the
/// address, so nothing can wake it early and the call is a timed sleep the
/// scheduler can actually park.
pub fn mk_idle_ms(ms: u64) -> i64 {
    // Lives across the call and is four-byte aligned, both of which the kernel
    // checks before it will park anyone.
    let word: u32 = 0;
    call_raw(N_MK_FUTEX_WAIT, [&word as *const u32 as u64, 0, ms, 0, 0, 0])
}
