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

//! What the kernel takes back when the ACPI capsule leaves (F-107), for a
//! restart to take again: the fixed power button, read by the kernel's poll
//! from this moment; every event enable, so nothing stays armed with no SCI
//! handler; and the Global Lock, when the capsule said it held it.
//! Its SCI line and ports go back with its other grants. A press the poll
//! reads while no capsule serves the button is the wiped power-off, run
//! from init's loop rather than the tick. The decisions are pure, in
//! `button_owner`, `leave_rule` and `hw::leave_events`; this is the glue.

use super::button_owner::give_back;
use super::leave_rule::{lock_at_exit, take_gap_press};
use crate::arch::power::PowerOff;
use crate::security::zerostate::terminate;
use crate::services::registry::lookup_service;
use crate::sys::serial::Line;

/// From the exit path for every pid. For the ACPI capsule's: the button
/// back, and whether it held the Global Lock, for the caller to disarm its
/// events (`arch::acpi_capsule`) and release the lock with them.
pub fn on_exit(pid: u32) -> Option<bool> {
    if pid == 0 || lookup_service("acpi").map(|e| e.pid) != Some(pid) {
        return None;
    }
    let button = give_back(pid);
    let lock = lock_at_exit(pid);
    let b: &[u8] = if button { b", the power button back with the kernel's poll" } else { b"" };
    let l: &[u8] = if lock { b", the Global Lock it held released" } else { b"" };
    Line::new().str(b"[ACPI] capsule left: its events disarmed").str(b).str(l).end();
    Some(lock)
}

/// From init's loop: the wiped power-off for a press in the gap.
pub fn serve_gap_press() {
    if take_gap_press() {
        Line::new().str(b"[ACPI] power button pressed while the ACPI capsule restarts").end();
        terminate(PowerOff::Shutdown)
    }
}
