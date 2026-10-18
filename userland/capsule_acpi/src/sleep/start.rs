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

//! \_S5 read at start, and again after any table loaded at run time, with
//! nothing run, and handed to the kernel: on a laptop whose \_S5 is a
//! method, the kernel's constant reader found none, and soft-off becomes
//! available from here on, even for a shutdown that never comes through
//! PREPARE_SLEEP. The kernel's own pair stands when it has one (F-111).

use crate::boot::evaluator;
use crate::host::CapsuleHost;
use crate::sys::acpi_sleep::{acpi_sleep, SOFT_OFF};
use crate::sys::log::Line;
use nonos_aml::load::Aml;
use nonos_aml::sleep::sleep_types;

pub fn read_s5(aml: &mut Aml, host: &mut CapsuleHost) {
    match sleep_types(aml, &mut evaluator(host), SOFT_OFF) {
        Some(t) => {
            let l = Line::new(b"\\_S5 SLP_TYP ").dec(u64::from(t.a)).str(b"/").dec(u64::from(t.b));
            match acpi_sleep(SOFT_OFF, t) {
                Ok(()) => l.end(),
                Err(e) => l.str(b"; the kernel refused it: ").dec(e.unsigned_abs()).end(),
            }
        }
        None => {
            let rest = b"soft-off rests on the kernel's constant reader (its posture line)";
            Line::new(b"no usable \\_S5 here: ").str(rest).end()
        }
    }
}
