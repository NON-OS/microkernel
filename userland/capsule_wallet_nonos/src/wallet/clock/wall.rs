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
//! The wall clock in unix seconds, for what is compared with chain time and
//! the meter's: capsule_payment's meter/refresh.rs reads the same clock.

use nonos_libc::mk_time_millis;

use super::unix::unix_s;

/// Now in unix seconds; 0 until the kernel's wall clock is set, while
/// MkTimeMillis answers -61 (src/syscall/microkernel/time.rs).
pub fn now_s() -> u64 {
    unix_s(mk_time_millis())
}
