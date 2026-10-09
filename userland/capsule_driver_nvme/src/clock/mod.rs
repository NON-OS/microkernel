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

mod budget;

use nonos_libc::mk_uptime_ms;

pub use budget::Budget;

/// The kernel's monotonic uptime, or None when the read fails.
pub fn uptime_ms() -> Option<u64> {
    let t = mk_uptime_ms();
    if t < 0 {
        None
    } else {
        Some(t as u64)
    }
}

/// A budget of `limit_ms` from now.
pub fn budget(limit_ms: u64) -> Budget {
    Budget::begin(uptime_ms(), limit_ms)
}
