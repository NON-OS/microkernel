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


//! What the kernel says about time and the broker's clients.

use nonos_libc::{mk_cap_check, mk_pid_alive, mk_time_millis, mk_uptime_ms};
use nonos_user_files::Host;

/// FileSystem: a native app that reaches the desktop's files. ForeignExec:
/// capsule_linux, which serves the person's files only to a program whose
/// signed policy grants FILES_USER.
const FILE_SYSTEM: u64 = 1 << 6;
const FOREIGN_EXEC: u64 = 1 << 32;

pub struct Desk;

impl Host for Desk {
    /// 0 until the wall clock is set: a file made then says 1970.
    fn now_ns(&self) -> u64 {
        u64::try_from(mk_time_millis()).unwrap_or(0).saturating_mul(1_000_000)
    }

    fn mono_ns(&self) -> u64 {
        u64::try_from(mk_uptime_ms()).unwrap_or(0).saturating_mul(1_000_000)
    }

    /// Asked of the kernel on every request; no verdict is kept, so a
    /// capability revoked is refused at the next request.
    fn may_use(&self, pid: u32) -> bool {
        pid != 0 && (mk_cap_check(pid, FILE_SYSTEM) || mk_cap_check(pid, FOREIGN_EXEC))
    }

    fn alive(&self, pid: u32) -> bool {
        mk_pid_alive(pid)
    }
}
