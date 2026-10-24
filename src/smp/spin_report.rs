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

//! The line a lock wait too long prints: where it waits, on which CPU and for
//! whom, and where the lock was taken.

use core::panic::Location;

use super::spin_holders::taker;
use super::spin_watch::same_chain;

/// The one line for a wait on `lock` at `site` that has lasted `ms`.
pub(super) fn report(lock: usize, site: &'static Location<'static>, sp: usize, ms: u64) {
    let mut line = crate::sys::serial::Line::new();
    line.str(b"[LOCK] waiting ms=").dec(ms).str(b" at ");
    line.str(site.file().as_bytes()).str(b":").dec(site.line() as u64);
    if let Some(cpu) = super::cpu_id::try_cpu_id() {
        let pid = crate::process::current_pid().unwrap_or(0);
        line.str(b" cpu=").dec(cpu as u64).str(b" pid=").hex(pid as u64);
    }
    match taker(lock) {
        Some((held_at, held_sp)) => {
            line.str(b" held from ").str(held_at.file().as_bytes());
            line.str(b":").dec(held_at.line() as u64);
            if same_chain(held_sp, sp) {
                line.str(b", likely re-taken by its own holder");
            }
        }
        None => _ = line.str(b" holder unknown"),
    }
    line.end();
}
