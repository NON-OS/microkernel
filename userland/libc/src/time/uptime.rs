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

use crate::syscall::{call_raw, N_MK_TIME_MONOTONIC};

/// Monotonic milliseconds since boot. Never moves backwards and is available
/// before the wall clock is set, so it is the source for bounded timeouts.
/// Read from the kernel's clock page when it has one for this process
/// (`MkClockPage`), otherwise by `MkTimeMonotonic`; the same milliseconds
/// either way.
pub extern "C" fn mk_uptime_ms() -> i64 {
    if let Some(ms) = super::clock_page_read::monotonic_ms() {
        return ms.min(i64::MAX as u64) as i64;
    }
    call_raw(N_MK_TIME_MONOTONIC, [0; 6])
}
