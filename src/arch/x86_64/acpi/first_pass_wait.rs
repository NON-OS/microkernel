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

//! L55, the lead's half of the ACPI capsule's first publish pass: init's
//! drivers wait for it on an ACPI boot whose capsule was spawned
//! (first_pass_rule::wait_needed), never past FIRST_PASS_WAIT_MS, and the
//! wait is said once, either way. Each look is first_pass_rule::look, pure.
//! Reached from init through crate::arch::acpi_first_pass_wait.

use crate::hardware::broker::acpi::{first_pass::done, first_pass_rule as rule};
use crate::sys::boot_log;

pub(crate) fn wait(spawned: bool) {
    if !rule::wait_needed(super::is_initialized(), spawned, done()) {
        return;
    }
    let start = crate::time::timestamp_millis();
    loop {
        let waited = crate::time::timestamp_millis().saturating_sub(start);
        match rule::look(done(), waited) {
            rule::Look::Wait => crate::sched::yield_now(),
            rule::Look::Done => {
                let said = alloc::format!("first publish pass in after {waited} ms");
                return boot_log::ok("ACPI", &said);
            }
            rule::Look::GaveUp => {
                let said = alloc::format!("no ACPI first pass in {waited} ms; drivers go on");
                return boot_log::warn(&said);
            }
        }
    }
}
