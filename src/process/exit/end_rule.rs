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

//! Which ends of a capsule the kernel names on the serial line, and in what
//! words. A driver that holds no Debug capability cannot print its own
//! absence or give-up line, so the kernel says it for it. Only `driver.`
//! names are told: the kernel spawns those itself, and their names say
//! nothing about what the person runs. Pure, so the rule is held on the host
//! (kernel_proofs).

/// The status `nonos_libc::bring_up` leaves with when no device was found.
pub const EXIT_ABSENT: i32 = 2;
/// The status it leaves with when a present device never came up.
pub const EXIT_GAVE_UP: i32 = 6;

const DRIVER_PREFIX: &str = "driver.";

/// Whether this end is worth a line: a driver's own exit with a status
/// other than zero. A fault already has the trap handler's line.
pub fn told(name: &str, status: i32, by_signal: bool) -> bool {
    !by_signal && status != 0 && name.starts_with(DRIVER_PREFIX)
}

/// The words for a status the bring-up policy gives meaning to.
pub const fn words(status: i32) -> &'static str {
    match status {
        EXIT_ABSENT => "no device present, not started",
        EXIT_GAVE_UP => "device present, bring-up failed and was given up",
        _ => "ended with an error",
    }
}
