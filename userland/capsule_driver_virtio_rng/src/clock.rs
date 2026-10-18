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

//! The boot uptime clock, which every wait here is measured by (never a
//! count of reads or yields, which runs at the scheduler's pace).

use nonos_libc::{mk_idle_ms, Deadline};

/// A wait's give-up test: false, after a millisecond's sleep, until `ms`
/// have passed since it was made.
pub fn expired_after(ms: u64) -> impl FnMut() -> bool {
    let deadline = Deadline::after_ms(ms);
    move || {
        if deadline.expired() {
            return true;
        }
        let _ = mk_idle_ms(1);
        false
    }
}
