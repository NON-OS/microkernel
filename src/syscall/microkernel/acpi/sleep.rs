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

//! `MkAcpiSleep(state, slp_typ_a, slp_typ_b) -> 0`: the ACPI capsule has
//! evaluated \_S5 (after \_TTS and \_PTS, when a shutdown is coming) and
//! hands its SLP_TYP values to the kernel's soft-off, which takes them when
//! its own constant reader found none (`s5_choice`). EINVAL for any state
//! but 5 or a value wider than three bits; ENODEV when the kernel never
//! read the ACPI tables.

use crate::hardware::broker::acpi::sleep_adopt::adopt_s5;
use crate::hardware::broker::acpi::sleep_request::{compare, valid, Before};
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NODEV};

pub fn sys_acpi_sleep(state: u64, a: u64, b: u64) -> i64 {
    if !valid(state, a, b) {
        return ERRNO_INVAL;
    }
    let now = (a as u8, b as u8);
    let Some(before) = adopt_s5(now.0, now.1) else { return ERRNO_NODEV };
    match compare(before, now) {
        Before::Same => {}
        Before::None => crate::log_info!(
            "[ACPI] \\_S5 from the interpreter: SLP_TYP {}/{}; soft-off is now available",
            now.0,
            now.1
        ),
        Before::Differed(ca, cb) => crate::log_warn!(
            "[ACPI] \\_S5 from the interpreter: SLP_TYP {}/{}; the constant reader's {}/{} stands",
            now.0,
            now.1,
            ca,
            cb
        ),
    }
    0
}
