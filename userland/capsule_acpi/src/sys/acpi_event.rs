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

//! MkAcpiEvent: a control-method button pressed, the lid, or the hottest
//! thermal zone, to the kernel. The numbers are the kernel's
//! (`broker::acpi::platform_event`).

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const EVENT: i64 = tag4(b"MAEV");

pub const POWER_BUTTON: u64 = 1;
pub const SLEEP_BUTTON: u64 = 2;
pub const LID: u64 = 3;
pub const THERMAL: u64 = 4;
pub const BATTERY_LOW: u64 = 5;
pub const BATTERY_CRITICAL: u64 = 6;
pub const BUTTON_TAKEN: u64 = 7;
pub const BRIGHTNESS: u64 = 8;
pub const GLOBAL_LOCK: u64 = 9;

pub const TENTHS_K_MASK: u64 = 0xFFFF;
pub const PASSIVE: u64 = 1 << 16;
pub const HOT: u64 = 1 << 17;
pub const CRITICAL: u64 = 1 << 18;

/// The input ring was busy; the kernel asks for the press again.
pub const AGAIN: Errno = -11;

pub fn acpi_event(kind: u64, value: u64) -> Result<(), Errno> {
    ok(mk_syscall_raw(EVENT, [kind, value, 0, 0, 0, 0])).map(|_| ())
}

/// BUTTON_TAKEN: true when a capsule served the button before this one,
/// which is then a restart.
pub fn button_taken() -> Result<bool, Errno> {
    ok(mk_syscall_raw(EVENT, [BUTTON_TAKEN, 0, 0, 0, 0, 0])).map(|v| v == 1)
}

/// The Global Lock held, or about to be given back. A refusal costs only
/// the release on this capsule's exit, so it is not reported.
pub fn lock_told(held: bool) {
    let _ = acpi_event(GLOBAL_LOCK, u64::from(held));
}
