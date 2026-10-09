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

//! The log line for an AP whose TSC_ADJUST was set, printed once it may
//! print (see `go_live`).

use super::tsc_adjust::{BOOT_VALUE, SET, WAS};
use crate::smp::MAX_CPUS;
use crate::sys::serial::Line;
use core::sync::atomic::Ordering;

/// Say on the log that an AP's register was set, once it may print.
pub(super) fn report(cpu_id: u32) {
    let cpu = cpu_id as usize;
    if cpu < MAX_CPUS && SET[cpu].load(Ordering::Acquire) {
        let mut l = Line::new();
        l.str(b"[SMP] ap=").dec(cpu as u64).str(b" TSC_ADJUST was ");
        signed(&mut l, WAS[cpu].load(Ordering::Relaxed)).str(b", set to the boot CPU's ");
        signed(&mut l, BOOT_VALUE.load(Ordering::Relaxed)).end();
    }
}

fn signed(l: &mut Line, raw: u64) -> &mut Line {
    let v = raw as i64;
    l.str(if v < 0 { b"-" } else { b"" }).dec(v.unsigned_abs())
}
