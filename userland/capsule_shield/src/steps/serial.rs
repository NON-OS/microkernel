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

//! Lines on the serial console, and the uptime they are stamped with.

/// One line on the serial console: what the service is doing, and when.
/// Names and times only, never a field of a request: no words, key or
/// address reaches the console.
pub fn serial(what: &str) {
    let line = format!("[shield {} ms] {what}\n", uptime());
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

pub(super) fn uptime() -> u64 {
    nonos_libc::mk_uptime_ms().max(0) as u64
}
