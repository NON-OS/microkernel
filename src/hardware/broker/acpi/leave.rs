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

use super::battery_watch::tick as battery_tick;
use super::button_owner::give_back;
use super::critical_watch::Action;
use super::leave_rule::{lock_at_exit, take_gap_press};
use super::thermal_watch::tick as thermal_tick;
use crate::arch::power::PowerOff;
use crate::security::zerostate::{terminate, terminate_critical};
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

/// From init's loop: the wiped power-off for a press in the gap, or for a
/// zone or battery whose grace ran out with no reading since (F-108).
pub fn serve_from_init() {
    if take_gap_press() {
        let why = b"[ACPI] power button pressed while the ACPI capsule restarts";
        Line::new().str(why).str(b"; _PTS(5) not run, no interpreter serves").end();
        terminate(PowerOff::Shutdown)
    }
    let now = crate::sys::clock::since_boot_ms();
    let late: [(Action, &[u8]); 2] =
        [(thermal_tick(now), b"thermal zone"), (battery_tick(now), b"battery")];
    if let Some((_, what)) = late.iter().find(|(a, _)| *a == Action::PowerOff) {
        let line = b" critical by its last reading, its grace over: powering off, _PTS(5) not run";
        Line::new().str(b"[ACPI] ").str(what).str(line).end();
        terminate_critical(PowerOff::Shutdown)
    }
}
