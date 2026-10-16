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

//! The first publish pass (L55): what init waits on, done before anything
//! slow. The reserved ranges and proximity domains; the host bridges'
//! readiness durations, then each hidden function's _PS0 and probe within
//! `FIRST_PASS_MS`, the rest left for after; the platform records in
//! _DEP order. Then the kernel is told, once.

use super::pass::{FirstPass, Step};
use super::state::Acpi;
use crate::sys::clock::now_ms;
use crate::sys::first_pass::first_pass_done;
use crate::sys::log::Line;

/// The hidden functions' share of the pass: well inside init's 5 s.
pub const FIRST_PASS_MS: u64 = 2_000;

pub fn first(acpi: &mut Acpi, pass: &mut FirstPass) {
    let until = now_ms().saturating_add(FIRST_PASS_MS);
    crate::reserved::publish(acpi);
    crate::numa::publish(acpi);
    crate::pci_firmware::find(acpi);
    let all = crate::dpower::reveal_hidden(acpi, until);
    pass.finish(Step::Hidden);
    crate::hotplug::learn(acpi);
    let Acpi { devices, aml, host, .. } = acpi;
    crate::platform::publish(devices, aml, host);
    pass.finish(Step::Records);
    if pass.raise() {
        let l = Line::new(b"first publish pass done");
        let l = if all { l } else { l.str(b"; hidden functions left for after it") };
        match first_pass_done() {
            Ok(()) => l.end(),
            Err(e) => l.str(b"; the kernel refused, ").dec(e.unsigned_abs()).end(),
        }
    }
}
