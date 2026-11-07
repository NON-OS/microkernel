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

//! The host clock the server reads. nsh/1 checks the enrolment's validity,
//! the grant's window and the idle timeout against wall-clock milliseconds,
//! so the session binds to the machine's real time, not its uptime. A guest
//! with no clock set reads zero, which no enrolment window covers; a guest
//! that serves nsh must keep real time (the VPS runbook pairs it with a
//! vTPM and NTP).

use nonos_libc::mk_time_millis;

/// Wall-clock milliseconds since the Unix epoch, or 0 if unset.
pub fn now() -> u64 {
    let ms = mk_time_millis();
    if ms < 0 {
        0
    } else {
        ms as u64
    }
}
