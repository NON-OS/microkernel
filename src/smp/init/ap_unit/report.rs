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

//! The lines the boot CPU prints about each AP it starts.

/// Parked and reset: no longer a CPU the machine waits for.
pub(super) fn abandoned(cpu_id: usize, reason: &[u8]) {
    line(cpu_id, b" abandoned at bring-up: ", reason);
}

/// Not parked, or not reset: still `Starting`, still a shootdown target.
pub(super) fn stuck(cpu_id: usize, reason: &[u8]) {
    line(cpu_id, b" stuck at bring-up: ", reason);
}

pub(super) fn line(cpu_id: usize, what: &[u8], reason: &[u8]) {
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SMP] cpu ").dec(cpu_id as u64);
    l.str(what).str(reason);
    l.end();
}

pub(super) fn report(cpu_id: usize, apic_id: u32, what: &[u8], detail: &[u8]) {
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SMP] ap=").dec(cpu_id as u64);
    l.str(b" apic=").dec(apic_id as u64);
    l.str(what);
    l.str(detail);
    l.end();
}
