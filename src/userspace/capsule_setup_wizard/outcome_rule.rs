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

//! What setup asked for, read from its exit status. The status is taken
//! first from the kept table, which no waiter consumes, then from the
//! process itself, then from the reap log. Pure, so the host proofs mount it.

/// The status setup exits with when the person chose to install NONOS.
const EXIT_INSTALLER: i32 = 3;
/* The status of a setup the person finished with another mode. */
const EXIT_DESKTOP: i32 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ended {
    /// Start the desktop.
    Desktop,
    /// Start the installer full screen; the desktop only if it ends without a restart.
    Installer,
    /* Setup never started, failed, or its status is gone: nobody chose. */
    Unfinished,
}

/// Setup has ended: its pid, and the status each place holds for it.
pub fn read(pid: u32, kept: Option<i32>, pcb: Option<i32>, logged: Option<i32>) -> (Ended, u8) {
    match kept.or(pcb).or(logged).filter(|_| pid != 0).and_then(parts) {
        Some((EXIT_INSTALLER, off)) => (Ended::Installer, off),
        Some((EXIT_DESKTOP, off)) => (Ended::Desktop, off),
        _ => (Ended::Unfinished, 0),
    }
}

/*
 * The kernel's reading of nonos_policy_proto::apps::exit_parts: a status
 * with bits above the apps byte, or a negative one, is not one setup wrote.
 */
fn parts(status: i32) -> Option<(i32, u8)> {
    if !(0..1 << 16).contains(&status) {
        return None;
    }
    Some((status & 0xFF, (status >> 8) as u8))
}
